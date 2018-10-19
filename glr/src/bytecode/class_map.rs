use super::Class;
use super::shared::*;
use super::shared::mem::{MemoryRange, CLASS_MAPPING};

pub enum ClassError {
    OutOfMemory,
    BadClassName,
}

pub struct ClassMapping {
    size: usize,
    capacity: usize,
    memory: MemoryRange,
    classes: *mut *mut Class,
}

impl ClassMapping {
    pub fn new() -> Option<Self> {
        try {
            let mut mapping = Self {
                size: 0,
                capacity: 8,
                classes: null_mut(),
                memory: MemoryRange::at(CLASS_MAPPING)?
            };
            mapping.classes = mapping.memory.as_ptr();
            mapping.grow_to(mapping.capacity)?;
            mapping
        }
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.size >= self.capacity && self.capacity >= self.memory.len()
    }

    fn grow_to(&mut self, capacity: usize) -> Option<()> {
        let growth = capacity.min(self.memory.len()) - self.capacity;
        let size = core::mem::size_of::<*mut Class>() * growth;
        self.memory.alloc_bytes(size).and_then(|bytes| unsafe {
            core::ptr::write_bytes(bytes, 0, size);
            Some(())
        })
    }

    fn hash(class_name: &str) -> usize {
        const FNV_PRIME: u32 = 16777619;
        const FNV_OFFSET: u32 = 2166136261;
        class_name
            .as_bytes().iter()
            .fold(FNV_OFFSET, |hash, &byte| (hash ^ byte as u32) * FNV_PRIME)
            as usize
    }

    pub fn insert(&mut self, class: *mut Class) -> Result<(), ClassError> {
        unsafe {
            match (*class).name() {
                Some(name) if !self.is_full() => self.put(name, &mut *class),
                Some(_) => Err(ClassError::OutOfMemory),
                None => Err(ClassError::BadClassName),
            }
        }
    }

    pub fn find(&self, class_name: &str) -> Option<&mut Class> {
        unsafe {
            if unlikely(self.size == 0) {
                return None
            }

            let mask = self.capacity - 1;
            let start_index = Self::hash(class_name) & mask;
            let mut index = start_index;
            let mut slot_class = self.classes.offset(index as isize);

            while !(*slot_class).is_null() && (**slot_class).is_called(class_name) {
                index = (index + 1) & mask;
                if index == start_index {
                    return None
                } else {
                    slot_class = self.classes.offset(index as isize);
                }
            }

            Some(&mut **slot_class)
        }
    }

    unsafe fn put(&mut self, class_name: &str, class: &mut Class) -> Result<(), ClassError> {
        if unlikely(class.next_class() > 0) {
            return Ok(())
        }

        self.size += 1;
        if self.size >= self.capacity {
            self.grow_to(self.capacity << 1)
                .ok_or(ClassError::OutOfMemory)?
        }

        let mask = self.capacity - 1;
        let mut index = Self::hash(class_name) & mask;

        for _ in 0..self.size {
            let slot_class = self.classes.offset(index as isize);
            if (*slot_class).is_null() {
                *slot_class = class;
                return Ok(())
            } else if (**slot_class).next_class() < class.next_class() {
                core::mem::swap(&mut *slot_class, &mut (class as *mut _));
            }
            class.bump_next_class();
            index = (index + 1) & mask;
        }

        Err(ClassError::OutOfMemory)
    }
}