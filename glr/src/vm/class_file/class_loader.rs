use super::Class;
use crate::os::pool::PoolAllocator;

const CLASS_MAP_RANGE: usize = 1 << 29;
const CLASS_MEM_RANGE: usize = 1 << 30;

#[inline(always)]
fn alloc_at(address_range: usize) -> Option<PoolAllocator> {
    PoolAllocator::new(address_range, (address_range << 1) - address_range)
}

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
                allocator: alloc_at(CLASS_MEM_RANGE)?,
                mapping: ClassMapping::at(CLASS_MAP_RANGE)?,
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
            let mut allocator = alloc_at(address_range)?;
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