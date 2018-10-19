use super::*;

impl<'a> ClassLoadable<'a, ()> for Class {
    fn load(_: (), reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
        Err(ClassError::OutOfMemory)
    }
}

impl<'a> ClassLoadable<'a, *mut Class> for Field {
    fn load(class: *mut Class, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
        Err(ClassError::OutOfMemory)
    }
}

impl<'a> ClassLoadable<'a, *mut Class> for Method {
    fn load(class: *mut Class, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError> {
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