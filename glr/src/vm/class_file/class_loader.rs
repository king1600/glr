use super::Class;
use crate::os::pool::PoolAllocator;

pub struct ClassLoader {
    memory: PoolAllocator,
    mapping: PoolAllocator,
    classes: *mut Class,
    capacity: usize,
    size: usize,
}

impl ClassLoader {
    pub fn new() -> Option<Self> {
        try {
            const CLASS_SIZE: usize = core::mem::size_of::<*mut Class>();

            let (size, capacity) = (0, 8);
            let memory = Self::alloc_pool_at(1 << 29)?;
            let mut mapping = Self::alloc_pool_at(1 << 30)?;
            let classes = mapping.alloc_bytes(CLASS_SIZE * capacity)? as *mut _;

            Self { size, capacity, classes, mapping, memory }
        }
    }

    #[inline(always)]
    fn alloc_pool_at(memory_range: usize) -> Option<PoolAllocator> {
        PoolAllocator::new(memory_range, (memory_range << 1) - memory_range)
    }

    pub fn load_class(&mut self, _name: &str, _bytes: &[u8]) -> Option<&mut Class> {
        None
    }
}