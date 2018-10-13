use super::*;
use core::mem::transmute;

pub struct Thread {
    #[cfg(unix)] handle: pthread_t,
    #[cfg(windows)] handle: HANDLE,
}

impl Drop for Thread {
    fn drop(&mut self) {
        self.join();
    }
}

impl Thread {
    pub fn create(func: extern "C" fn(usize) -> usize, arg: usize) -> Self {
        unsafe { thread_impl::create(transmute(func), arg as *mut c_void) }
    }

    #[inline]
    pub fn yield_now() {
        unsafe { thread_impl::yield_now(); }
    }

    #[inline]
    pub fn join(&mut self) {
        unsafe { thread_impl::join(&self); }
    }

    #[inline]
    pub fn exit() {
        unsafe { thread_impl::exit(); }
    }

    #[inline]
    pub fn current() -> Self {
        unsafe { thread_impl::current() }
    }
}

#[cfg(unix)]
mod thread_impl {
    use super::*;

    #[inline]
    pub unsafe fn create(func: extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> Thread {
        let mut thread_id = 0;
        pthread_create(&mut thread_id, null_mut(), func, arg);
        Thread { handle: thread_id }
    }

    #[inline]
    pub unsafe fn join(thread: &Thread) {
        pthread_join(thread.handle, null_mut());
    }

    #[inline]
    pub unsafe fn current() -> Thread {
        Thread { handle: pthread_self() }
    }

    #[inline]
    pub unsafe fn exit() {
        pthread_exit(null_mut());
    }

    #[inline]
    pub unsafe fn yield_now() {
        sched_yield();
    }
}

#[cfg(windows)]
mod thread_impl {
    use super::*;

    #[inline]
    pub unsafe fn create(func: extern "system" fn(*mut c_void) -> DWORD, arg: *mut c_void) -> Thread {
        let mut thread_id = 0;
        Thread { handle: CreateThread(null_mut(), 0, Some(func), arg, 0, &mut thread_id) }
    }

    #[inline]
    pub unsafe fn join(thread: &Thread) {
        WaitForSingleObject(thread.handle, INFINITE);
        CloseHandle(thread.handle);
    }

    #[inline]
    pub unsafe fn current() -> Thread {
        Thread { handle: GetCurrentThread() }
    }

    #[inline]
    pub unsafe fn yield_now() {
        SwitchToThread();
    }

    #[inline]
    pub unsafe fn exit() {
        ExitThread(0);
    }
}