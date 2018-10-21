use super::{Class, ClassResult, ClassError};
use super::{Reader, Mapping, Mappable, Hash32};
use super::shared::mem::{MemoryRange, CLASS_MEMORY, CLASS_MAPPING};

const DEFAULT_CLASSES: usize = 8;

pub struct ClassLoader {
    mapping: MemoryRange,
    pub memory: MemoryRange,
    classes: Mapping<str, Class>,
}

pub trait ClassLoadable<'a, T>: Sized {
    fn load(
        root: T,
        reader: &mut Reader<'a>,
        loader: &mut ClassLoader,
    ) -> Result<Self, ClassError>;
}

impl ClassLoader {
    pub fn new() -> ClassResult<Self> {
        let class_loader: Option<Self> = try {
            let memory = MemoryRange::at(CLASS_MEMORY)?;
            let mut mapping = MemoryRange::at(CLASS_MAPPING)?;
            let classes = Mapping::from(&mut mapping, DEFAULT_CLASSES)?;
            Self { memory, mapping, classes }
        };
        class_loader.ok_or(ClassError::OutOfMemory)
    }

    #[inline]
    pub fn alloc<T: Sized>(&mut self, value: T) -> ClassResult<*mut T> {
        self.memory.alloc(value).ok_or(ClassError::OutOfMemory)
    }

    #[inline]
    pub fn alloc_many<T: Sized>(&mut self, amount: usize) -> ClassResult<*mut T> {
        self.memory.alloc_many(amount).ok_or(ClassError::OutOfMemory)
    }

    #[inline]
    pub fn alloc_bytes(&mut self, size: usize) -> ClassResult<*mut u8> {
        self.memory.alloc_bytes(size).ok_or(ClassError::OutOfMemory)
    }

    #[inline]
    pub fn alloc_mapping<K, V: Mappable<K>>(&mut self, capacity: usize)
        -> Result<Mapping<K, V>, ClassError>
        where K: PartialEq + Hash32, V: Mappable<K> {
        Mapping::from(&mut self.memory, capacity).ok_or(ClassError::OutOfMemory)
    }

    #[inline]
    pub fn find(&self, class_name: &str) -> Option<&mut Class> {
        self.classes.find(class_name)
    }

    pub fn load_class(&mut self, bytes: &[u8]) -> ClassResult<*mut Class> {
        unsafe {
            let class = Class::load((), &mut bytes.into(), self)?;
            let class = self.alloc(class)?;

            self.classes.insert(class).or_else(|| {
                if self.classes.expand(self.mapping.len()) {
                    self.classes.insert(class)
                } else {
                    None
                }
            }).ok_or(ClassError::OutOfMemory)?;

            Ok(class)
        }
    }
}