use super::{mem::*, info::*};
use core::mem::{drop, size_of, forget};
use core::intrinsics::{likely, unlikely};

pub struct PoolAllocator {
    top: usize,
    limit: usize,
    memory: usize,
    capacity: usize,
    page_size: usize,
}

impl Drop for PoolAllocator {
    fn drop(&mut self) {
        while self.capacity != 0 {
            let page_size = self.capacity.min(self.page_size);
            drop(unsafe { Page::from(self.memory, page_size) });
            self.capacity -= page_size;
            self.memory += page_size;
        }
    }
}

impl PoolAllocator {
    pub fn new(addr: usize, limit: usize) -> Option<Self> {
        try {
            let page_size = SYS_INFO.huge_page_size.max(SYS_INFO.page_size).min(limit);
            let (top, (memory, capacity)) = (0, Self::alloc_page(addr, page_size)?);
            Self { top, limit, memory, capacity, page_size }
        }
    }

    #[inline]
    pub fn alloc_bytes(&mut self, size: usize) -> Option<*mut u8> {
        self.alloc_raw(size).and_then(|bytes| Some(bytes as *mut u8))
    }

    #[inline]
    pub fn alloc<T: Sized>(&mut self) -> Option<&mut T> {
        self.alloc_bytes(size_of::<T>())
            .and_then(|bytes| Some(unsafe {
                &mut *(bytes as *mut _)
            }))
    }

    #[inline(always)]
    fn bump_alloc(&mut self, bytes: usize) -> usize {
        let ptr = self.top;
        self.top += bytes;
        ptr
    }

    #[inline]
    fn alloc_page(addr: usize, size: usize) -> Option<(usize, usize)> {
        Page::new(addr, size, PAGE_READ | PAGE_WRITE | PAGE_HUGE).and_then(|page| {
            let page_info = (page.addr, page.size);
            forget(page);
            Some(page_info)
        })
    }

    fn alloc_raw(&mut self, bytes: usize) -> Option<usize> {
        unsafe {
            if unlikely(self.top + bytes > self.capacity) {
                let growth = (self.capacity + self.page_size).min(self.limit) - self.capacity;
                if likely(growth > 0) {
                    Self::alloc_page(self.memory + self.capacity, growth).and_then(|_| {
                        self.capacity += growth;
                        Some(self.bump_alloc(bytes))
                    })
                } else {
                    None
                }
            } else {
                Some(self.bump_alloc(bytes))
            }
        }
    }
}