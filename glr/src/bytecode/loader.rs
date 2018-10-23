use super::TypeSize;
use super::{Mappable, Mapping, Hash32};
use super::{Reader, ClassError, ClassResult, ClassLoader, ClassLoadable};
use super::{Class, ClassFile, Field, FieldContext, Method, Const, ConstPool};

use core::mem::transmute;
use core::ptr::null_mut;
use core::ptr::copy_nonoverlapping as memcpy;

const CLASS_TYPE_ENUM:   u8 = 0;
const CLASS_TYPE_STRUCT: u8 = 1;
const CLASS_TYPE_MODULE: u8 = 2;

impl<'a> ClassLoadable<'a, ()> for Class {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self> {
        const CLASS_FILE_HEADER: &'static [u8; 4] = b"$GLR";

        // read class magic (first 4 bytes = "$GLR")
        let magic = unsafe { transmute(*CLASS_FILE_HEADER) };
        if reader.read::<u32>().ok_or(ClassError::BadClassMagic)? != magic {
            return Err(ClassError::BadClassMagic)
        }

        // read class class type, access modifier and class const pool
        let class_type = reader.read::<u8>().ok_or(ClassError::BadClassType)?;
        let access = reader.read::<u8>().ok_or(ClassError::BadAccessModifier)?;
        let const_pool = ConstPool::load((), reader, loader)?;

        // read class fields using class type and class methods using bytecode size
        let code_size = reader.read::<u32>().ok_or(ClassError::BadCodeSize)? as usize;
        let fields = load_mapped::<u8, u16, str, Field>(class_type, ClassError::BadFieldSize, reader, loader)?;
        let methods = load_mapped::<usize, u16, str, Method>(code_size, ClassError::BadMethodSize, reader, loader)?;
        
        // read and allocate bytecode data
        let code_data = reader.read_bytes(code_size).ok_or(ClassError::BadCodeData)?;
        let bytecode = loader.alloc_bytes_exec(code_size)?;
        unsafe { memcpy(code_data.as_ptr(), bytecode, code_size) };

        // create the class file
        let class_file = ClassFile {
            access,
            fields,
            methods,
            bytecode,
            const_pool,
            next_class: 0,
        };

        // wrap the class file into the designated class type
        match class_type {
            CLASS_TYPE_ENUM => Ok(Class::Enum(class_file)),
            CLASS_TYPE_STRUCT => Ok(Class::Struct(class_file)),
            CLASS_TYPE_MODULE => Ok(Class::Module(class_file)),
            _ => Err(ClassError::BadClassType)
        }
    }
}

impl<'a> ClassLoadable<'a, ()> for ConstPool {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self> {
        match reader.read::<u16>().ok_or(ClassError::BadConstSize)? as usize {
            0 => Err(ClassError::BadConstSize), // 1 constant required for class file name
            num_consts => unsafe {
                let mut const_pool = ConstPool::new(loader.alloc_many(num_consts)?, num_consts);
                for index in 0..num_consts {
                    let constant = Const::load((), reader, loader)?;
                    *const_pool.as_slice_mut().get_unchecked_mut(index) = constant;
                }
                Ok(const_pool)
            }
        }
    }
}

impl<'a> ClassLoadable<'a, ()> for Const {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self> {        
        let (type_size, is_string) = TypeSize::extract(reader.read::<u8>().ok_or(ClassError::BadConstType)?);
        let type_size = type_size.ok_or(ClassError::BadConstType)?;

        if is_string == 1 {
            let string_size = match read_const_num(type_size, reader)? {
                Const::UInt(string_size) => string_size,
                _ => return Err(ClassError::BadConstType)
            } as usize;

            let bytes = reader.read_bytes(string_size).ok_or(ClassError::BadConstData)?;
            let string = loader.alloc_bytes(string_size)?;
            unsafe { memcpy(bytes.as_ptr(), string, string_size) };
            Ok(Const::Str(string as *const _, string_size))
        } else {
            read_const_num(type_size, reader)
        }
    }
}

impl<'a> ClassLoadable<'a, u8> for Field {
    fn load(class_type: u8, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self>  {
        let context = FieldContext {
            next_field: 0,
            class: null_mut(),
        };

        match class_type {
            CLASS_TYPE_MODULE => {
                let module = reader.read::<u16>().ok_or(ClassError::BadConstIndex)?;
                Ok(Field::Module(context, module))
            },

            CLASS_TYPE_STRUCT => {
                let field_name = reader.read::<u16>().ok_or(ClassError::BadConstIndex)?;
                let field_type = reader.read::<u16>().ok_or(ClassError::BadConstIndex)?;
                Ok(Field::Struct(context, field_name, field_type))
            },

            CLASS_TYPE_ENUM => {
                let name = reader.read::<u16>().ok_or(ClassError::BadConstIndex)?;
                let num_values = reader.read::<u16>().ok_or(ClassError::BadEnumSize)?;
                let field = Field::Enum(context, name, None);

                (0..num_values).fold(Ok((field, None)), |fields: ClassResult<(Field, Option<*mut Field>)>, _| unsafe {
                    let (mut head, current) = fields?;
                    let enum_name = reader.read::<u16>().ok_or(ClassError::BadEnumField)?;
                    let enum_field = loader.alloc(Field::Enum(context, enum_name, None))?;

                    // set the previous enum field's next to point to the created enum_field
                    let field = current.unwrap_or(&mut head as *mut _);
                    if let Some(next_field) = (*field).next_field_mut() {
                        *next_field = Some(enum_field);
                    }

                    Ok((head, Some(enum_field)))
                }).and_then(|(field, _)| Ok(field))
            },
            _ => Err(ClassError::BadClassType)
        }
    }
}

impl<'a> ClassLoadable<'a, usize> for Method {
    fn load(code_size: usize, reader: &mut Reader<'a>, _loader: &mut ClassLoader) -> ClassResult<Self> {
        let name = reader.read::<u16>().ok_or(ClassError::BadConstIndex)?;
        let access = reader.read::<u8>().ok_or(ClassError::BadAccessModifier)?;
        let (type_size, _) = TypeSize::extract(reader.read::<u8>().ok_or(ClassError::BadCodePos)?);

        // read code pos and check if in the code range
        let type_size = type_size.ok_or(ClassError::BadCodePos)?;
        let code_pos = match read_const_num(type_size, reader)? {
            Const::UInt(code_pos) if code_pos as usize <= code_size => code_pos,
            _ => return Err(ClassError::BadCodePos)
        };

        Ok(Method {
            name,
            access,
            code_pos,
            next_method: 0,
            class: null_mut(),
        })
    }
}

fn read_const_num<'a>(type_size: TypeSize, reader: &mut Reader<'a>) -> ClassResult<Const> {
    Ok(match type_size {
        TypeSize::U8 => Const::UInt(reader.read::<u8>().ok_or(ClassError::BadConstData)? as u64),
        TypeSize::U16 => Const::UInt(reader.read::<u16>().ok_or(ClassError::BadConstData)? as u64),
        TypeSize::U32 => Const::UInt(reader.read::<u32>().ok_or(ClassError::BadConstData)? as u64),
        TypeSize::U64 => Const::UInt(reader.read::<u64>().ok_or(ClassError::BadConstData)?),
        TypeSize::I32 => Const::Int(reader.read::<i32>().ok_or(ClassError::BadConstData)? as i64),
        TypeSize::I64 => Const::Int(reader.read::<i64>().ok_or(ClassError::BadConstData)?),
        TypeSize::F32 => Const::Float(reader.read::<f32>().ok_or(ClassError::BadConstData)? as f64),
        TypeSize::F64 => Const::Float(reader.read::<f64>().ok_or(ClassError::BadConstData)?),
    })
}

fn load_mapped<'a, Root, Size, Key, Value>(
    root: Root,
    error: ClassError,
    reader: &mut Reader<'a>,
    loader: &mut ClassLoader,
) -> ClassResult<Option<Mapping<Key, Value>>> where
    Root: Copy + Clone,
    Size: Sized, usize: From<Size>,
    Key: ?Sized + PartialEq + Hash32,
    Value: Sized + Mappable<Key> + ClassLoadable<'a, Root>,
{
    match usize::from(reader.read::<Size>().ok_or(error)?) {
        0 => Ok(None),
        num_items => {
            let mut mapping = Mapping::from(&mut loader.memory, num_items).ok_or(ClassError::OutOfMemory)?;
            for _ in 0..num_items {
                let item = Value::load(root, reader, loader)?;
                let item = loader.alloc(item)?;
                mapping.insert(item).ok_or(ClassError::OutOfMemory)?;
            }
            Ok(Some(mapping))
        }
    }
}