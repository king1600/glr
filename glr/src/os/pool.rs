use super::{mem::*, info::*};

pub struct GlobalMemory {
    top: usize,
    memory: usize,
    capacity: usize,
    page_size: usize,
}

impl Drop for GlobalMemory {
    fn drop(&mut self) {
        while self.capacity != 0 {
            core::mem::drop(unsafe { Page::from(self.memory, self.page_size) });
            self.capacity -= self.page_size;
            self.memory += self.page_size;
        }
    }
}

impl GlobalMemory {
    pub fn new(addr: usize) -> Option<Self> {
        let page_size = SYS_INFO.huge_page_size.max(SYS_INFO.page_size);
        Self::alloc_page(addr, page_size).and_then(|(memory, capacity)| {
            Some(Self {
                top: 0,
                memory,
                capacity,
                page_size,
            })
        })
    }

    #[inline]
    pub fn alloc_bytes(&mut self, size: usize) -> Option<*mut u8> {
        (unsafe { self.alloc_raw(size) })
            .and_then(|bytes| Some(bytes as *mut u8))
    }

    #[inline]
    pub fn alloc<T: Sized>(&mut self) -> Option<&mut T> {
        self.alloc_bytes(core::mem::size_of::<T>()).and_then(|addr| {
            Some(unsafe {
                &mut *(addr as *mut _)
            })
        })
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
            core::mem::forget(page);
            Some(page_info)
        })
    }

    unsafe fn alloc_raw(&mut self, bytes: usize) -> Option<usize> {
        if core::intrinsics::unlikely(self.top + bytes > self.capacity) {
            Self::alloc_page(self.memory + self.capacity, self.page_size).and_then(|_| {
                self.capacity += self.page_size;
                Some(self.bump_alloc(bytes))
            })
        } else {
            Some(self.bump_alloc(bytes))
        }
    }
}