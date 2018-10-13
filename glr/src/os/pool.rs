use super::mem::*;

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