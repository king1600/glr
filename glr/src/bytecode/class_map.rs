use super::Class;
use super::shared::*;
use super::shared::mem::{MemoryRange, CLASS_MAPPING};

pub enum ClassInsertError {
    OutOfMemory,
    InvalidClassName,
}

pub struct ClassMapping {
    size: usize,
    capacity: usize,
    classes: *mut *mut Class,
    allocator: MemoryRange,
}

impl ClassMapping {
    pub fn new() -> Option<Self> {
        try {
            let (size, capacity) = (0, 8);
            let mut allocator = MemoryRange::at(CLASS_MAPPING)?;
            let classes = Self::alloc_classes(&mut allocator, capacity)?;
            Self { size, capacity, classes, allocator }
        }
    }

    #[inline]
    fn alloc_classes(allocator: &mut MemoryRange, capacity: usize) -> Option<*mut *mut Class> {
        let size = core::mem::size_of::<*mut Class>() * capacity;
        allocator.alloc_bytes(size).and_then(|bytes| Some(unsafe {
            core::ptr::write_bytes(bytes, 0, size);
            bytes as *mut *mut _
        }))
    } 

    #[inline]
    fn hash_class_name(name: &str) -> u32 {
        const FNV_PRIME: u32 = 16777619;
        const FNV_OFFSET: u32 = 2166136261;
        name.as_bytes().iter().fold(FNV_OFFSET, |hash, &byte| (hash ^ byte as u32) * FNV_PRIME)
    }

    #[inline]
    fn has_open_slots(&self) -> bool {
        self.size < self.capacity && self.capacity <= self.allocator.len()
    }

    pub fn insert(&mut self, class: *mut Class) -> Result<(), ClassInsertError> {
        unsafe {
            match (*class).name() {
                Some(name) if self.has_open_slots() => self.insert_inner(name, &mut *class),
                None => Err(ClassInsertError::InvalidClassName),
                _ => Err(ClassInsertError::OutOfMemory),
            }
        }
    }

    pub fn find(&self, class_name: &str) -> Option<&mut Class> {
        unsafe {
            if unlikely(self.size == 0) {
                return None
            }

            let mask = self.capacity - 1;
            let start_index = Self::hash_class_name(class_name) as usize & mask;
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

    unsafe fn insert_inner(&mut self, class_name: &str, class: &mut Class) -> Result<(), ClassInsertError> {
        if unlikely(class.next_class() > 0) {
            return Ok(())
        }
        
        self.size += 1;
        if self.size >= self.capacity {
            let old_capacity = self.capacity;
            self.capacity = (self.capacity << 1).min(self.allocator.len());
            Self::alloc_classes(&mut self.allocator, self.capacity - old_capacity)
                .ok_or(ClassInsertError::OutOfMemory)?;
        }

        let mask = self.capacity - 1;
        let mut index = Self::hash_class_name(class_name) as usize & mask;

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

        Err(ClassInsertError::OutOfMemory)
    }
}