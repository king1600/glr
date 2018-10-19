use super::*;
use super::shared::mem::{MemoryRange, CLASS_MEMORY};

pub struct ClassLoader {
    memory: MemoryRange,
    mapping: ClassMapping,
}

pub trait ClassLoadable<'a, T>: Sized {
    fn load(root: T, reader: &mut Reader<'a>, loader: &mut ClassLoader) -> Result<Self, ClassError>;
}

impl ClassLoader {
    pub fn new() -> Option<Self> {
        try {
            let mapping = ClassMapping::new()?;
            let memory = MemoryRange::at(CLASS_MEMORY)?;
            Self { memory, mapping }
        }
    }

    #[inline]
    pub fn alloc<T: Sized>(&mut self) -> Result<*mut T, ClassError> {
        self.memory.alloc().ok_or(ClassError::OutOfMemory)
    }

    #[inline]
    pub fn alloc_bytes(&mut self, size: usize) -> Result<*mut u8, ClassError> {
        self.memory.alloc_bytes(size).ok_or(ClassError::OutOfMemory)
    }

    pub fn load_class(&mut self, bytes: &[u8]) -> Result<&mut Class, ClassError> {
        unsafe {
            let class = self.alloc::<Class>()?;
            *class = Class::load((), &mut bytes.into(), self)?;
            self.mapping.insert(class)?;
            Ok(&mut *class)
        }
    }
}