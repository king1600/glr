use super::Class;
use crate::os::pool::{PoolAllocator, offsets};

pub struct ClassLoader {
    mapping: ClassMapping,
    allocator: PoolAllocator,
}

struct ClassMapping {
    size: usize,
    capacity: usize,
    classes: *mut Class,
    allocator: PoolAllocator,
}

impl ClassLoader {
    pub fn new() -> Option<Self> {
        try {
            Self {
                mapping: ClassMapping::at(offsets::CLASS_MAPPING)?,
                allocator: PoolAllocator::alloc_at(offsets::CLASS_MEMORY)?,
            }
        }
    }
}

impl ClassMapping {
    pub fn at(address_range: usize) -> Option<ClassMapping> {
        const CLASS_SIZE: usize = core::mem::size_of::<*mut Class>();

        try {
            let size = 0;
            let capacity = 8;
            let mut allocator = PoolAllocator::alloc_at(address_range)?;
            let classes = allocator.alloc_bytes(CLASS_SIZE * capacity)? as *mut _;
            Self { size, capacity, classes, allocator }
        }
    }

    pub fn find(&mut self, _class_name: &str) -> Option<&mut Class> {
        None
    }

    pub fn insert(&mut self, _class: *mut Class) -> Option<&mut Class> {
        None
    }
}