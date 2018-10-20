use super::*;

impl<'a> ClassLoadable<'a, ()> for Class {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
        const CLASS_FILE_HEADER: &'static [u8; 4] = b"$GLR";

        const CLASS_TYPE_ENUM:   u8 = 0;
        const CLASS_TYPE_STRUCT: u8 = 1;
        const CLASS_TYPE_MODULE: u8 = 2;

        // read class magic (first 4 bytes = "$GLR")
        let magic = unsafe { core::mem::transmute(*CLASS_FILE_HEADER) };
        if reader.read::<u32>().ok_or(ClassError::BadClassMagic)? != magic {
            return Err(ClassError::BadClassMagic)
        }

        // read class access modifier, class type and class const pool
        let access = reader.read::<u8>().ok_or(ClassError::BadAccessModifier)?;
        let class_type = reader.read::<u8>().ok_or(ClassError::BadClassType)?;
        let const_pool = ConstPool::load((), reader, loader)?;

        // read class fields and methods using class_type
        let fields = load_many::<u8, Field, u16>(class_type, ClassError::BadFieldSize, reader, loader)?;
        let methods = load_many::<u8, Method, u16>(class_type, ClassError::BadMethodSize, reader, loader)?;

        let class_file = ClassFile {
            access,
            fields,
            methods,
            const_pool,
            next_class: 0,
        };

        match class_type {
            CLASS_TYPE_ENUM => Ok(Class::Enum(class_file)),
            CLASS_TYPE_STRUCT => Ok(Class::Struct(class_file)),
            CLASS_TYPE_MODULE => Ok(Class::Module(class_file)),
            _ => Err(ClassError::BadClassType)
        }
    }
}

impl<'a> ClassLoadable<'a, u8> for Field {
    fn load(class_type: u8, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
        Err(ClassError::OutOfMemory)
    }
}

impl<'a> ClassLoadable<'a, u8> for Method {
    fn load(class_type: u8, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
        Err(ClassError::OutOfMemory)
    }
}

impl<'a> ClassLoadable<'a, ()> for ConstPool {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
        Err(ClassError::OutOfMemory)
    }
}

impl<'a> ClassLoadable<'a, ()> for Const {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
        Err(ClassError::OutOfMemory)
    }
}

fn load_many<'a, R, T, S>(
    root: R,
    error: ClassError,
    reader: &mut Reader<'a>,
    loader: &mut ClassLoader
) -> Result<Option<*mut T>, ClassError> where
    R: Copy + Clone,
    S: Sized, usize: From<S>,
    T: Sized + Nextable + ClassLoadable<'a, R>,
{
    match usize::from(reader.read::<S>().ok_or(error)?) {
        0 => Ok(None),
        num_items => unsafe {
            let num_bytes = core::mem::size_of::<T>() * num_items;
            let values = loader.alloc_bytes(num_bytes)? as *mut T;

            for index in 0..num_items {
                let value_ptr = values.offset(index as isize);
                *value_ptr = T::load(root, reader, loader)?;
                if index > 0 {
                    (*values.offset(-1)).set_next(value_ptr)
                }
            }

            Ok(Some(values))
        }
    }
}