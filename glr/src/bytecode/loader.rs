use super::{Mappable, Mapping, Hash32};
use super::{Reader, ClassError, ClassResult, ClassLoader, ClassLoadable};
use super::{Class, ClassFile, Field, FieldContext, Method, Const, ConstPool};

const CLASS_TYPE_ENUM:   u8 = 0;
const CLASS_TYPE_STRUCT: u8 = 1;
const CLASS_TYPE_MODULE: u8 = 2;

impl<'a> ClassLoadable<'a, ()> for Class {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self> {
        const CLASS_FILE_HEADER: &'static [u8; 4] = b"$GLR";

        // read class magic (first 4 bytes = "$GLR")
        let magic = unsafe { core::mem::transmute(*CLASS_FILE_HEADER) };
        if reader.read::<u32>().ok_or(ClassError::BadClassMagic)? != magic {
            return Err(ClassError::BadClassMagic)
        }

        // read class class type, access modifier and class const pool
        let class_type = reader.read::<u8>().ok_or(ClassError::BadClassType)?;
        let access = reader.read::<u8>().ok_or(ClassError::BadAccessModifier)?;
        let const_pool = ConstPool::load((), reader, loader)?;

        // read class fields and methods using class_type
        let fields = load_mapped::<u8, u16, str, Field>(class_type, ClassError::BadFieldSize, reader, loader)?;
        let methods = load_mapped::<u8, u16, str, Method>(class_type, ClassError::BadMethodSize, reader, loader)?;

        // create the class file
        let class_file = ClassFile {
            access,
            fields,
            methods,
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

impl<'a> ClassLoadable<'a, u8> for Field {
    fn load(class_type: u8, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self>  {
        let context = FieldContext {
            next_field: 0,
            class: core::ptr::null_mut(),
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

impl<'a> ClassLoadable<'a, u8> for Method {
    fn load(class_type: u8, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self> {
        Err(ClassError::OutOfMemory)
    }
}

impl<'a> ClassLoadable<'a, ()> for ConstPool {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self> {
        Err(ClassError::OutOfMemory)
    }
}

impl<'a> ClassLoadable<'a, ()> for Const {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> ClassResult<Self> {
        Err(ClassError::OutOfMemory)
    }
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