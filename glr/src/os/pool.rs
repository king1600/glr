use super::mem::*;

pub mod offsets {
    pub const STDERR_BUFFER: usize = 1 << 14; // 16kb of addressable memory
    pub const STDOUT_BUFFER: usize = 1 << 15; // 32kb of addressable memory
    pub const CLASS_MAPPING: usize = 1 << 29; // 512mb of addressable memory
    pub const CLASS_MEMORY:  usize = 1 << 30; // 1gb of addressable memory
}

pub struct PoolAllocator {
    top: usize,
    inner: Page,
}

impl PoolAllocator {
    pub fn new(from: usize, to: usize) -> Option<PoolAllocator> {
        Page::new(from, to - from, PAGE_READ | PAGE_WRITE | PAGE_HUGE).and_then(|page| {
            Some(Self { top: 0, inner: page })
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn taken_bytes(&self) -> &[u8] {
        &self.as_bytes()[..self.top]
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    #[inline]
    pub unsafe fn reset(&mut self) {
        self.top = 0;
    }

    #[inline(always)]
    pub fn alloc_at(address_range: usize) -> Option<PoolAllocator> {
        Self::new(address_range, address_range << 1)
    }

    #[inline]
    pub fn alloc<T: Sized>(&mut self) -> Option<&mut T> {
        self.alloc_bytes(core::mem::size_of::<T>())
            .and_then(|ptr| Some(unsafe { &mut *(ptr as *mut _) }))
    }

    pub fn alloc_bytes(&mut self, bytes: usize) -> Option<*mut u8> {
        unsafe {
            if core::intrinsics::likely(self.top + bytes < self.inner.len()) {
                let ptr = self.top;
                self.top += bytes;
                Some(ptr as *mut u8)
            } else {
                None
            }
        }
    }
}