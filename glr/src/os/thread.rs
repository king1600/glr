use super::*;

pub struct Thread {
    #[cfg(unix)] handle: pthread_t,
    #[cfg(windows)] handle: HANDLE,
}

impl Drop for Thread {
    fn drop(&mut self) {
        self.join();
    }
}

#[cfg(unix)]
impl Thread {
    pub fn create(func: extern "C" fn(usize) -> usize, arg: usize) -> Self {
        Self {
            handle: unsafe {
                let mut handle = 0;
                let func = core::mem::transmute(func);
                pthread_create(&mut handle, null_mut(), func, arg as *mut c_void);
                handle
            }
        }
    }

    #[inline]
    pub fn current() -> Thread {
        Self { handle: unsafe { pthread_self() } }
    }

    #[inline]
    pub fn exit() {
        unsafe { pthread_exit(null_mut()); }
    }

    #[inline]
    pub fn yield_now() {
        unsafe { sched_yield(); }
    }

    pub fn join(&mut self) {
        unsafe { pthread_join(self.handle, null_mut()); }
    }
}

#[cfg(windows)]
impl Thread {
    pub fn create(func: extern "C" fn(usize) -> usize, arg: usize) -> Self {
        Self {
            handle: unsafe {
                let mut id = 0;
                let func = core::mem::transmute(func);
                CreateThread(null_mut(), 0, Some(func), arg as *mut c_void, 0, &mut id)
            }
        }
    }

    #[inline]
    pub fn current() -> Thread {
        Self { handle: unsafe { GetCurrentThread() } }
    }

    #[inline]
    pub fn exit() {
        unsafe { ExitThread(0); }
    }

    #[inline]
    pub fn yield_now() {
        unsafe { SwitchToThread(); }
    }

    pub fn join(&mut self) {
        unsafe {
            WaitForSingleObject(self.handle, INFINITE);
            CloseHandle(self.handle);
        }
    }
}