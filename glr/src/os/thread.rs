use self::threading::*;

#[repr(C)]
pub struct Thread {
    handle: ThreadHandle,
}

impl Thread {
    #[inline]
    pub fn new(func: extern "system" fn(usize) -> usize, arg: usize) -> Thread {
        Thread { handle: unsafe { thread_create(func as usize, arg) } }
    }

    #[inline]
    pub fn current() -> Thread {
        Thread { handle: unsafe { thread_current() } }
    }

    #[inline]
    pub fn exit() {
        unsafe { thread_exit(); }
    }

    #[inline]
    pub fn yield_now() {
        unsafe { thread_yield(); }
    }
}

#[repr(C)]
pub struct Tls {
    index: u32,
}

impl Drop for Tls {
    fn drop(&mut self) {
        unsafe { tls_free(self.index); }
    }
}

impl Tls {
    #[inline]
    pub fn new() -> Tls {
        Tls { index: unsafe { tls_alloc() } }
    }

    #[inline]
    pub fn get(&self) -> usize {
        unsafe { tls_get(self.index) }
    }

    #[inline]
    pub fn set(&self, value: usize) {
        unsafe { tls_set(self.index, value); }
    }
}

#[cfg(unix)]
mod threading {
    use super::super::Handle;
    pub type ThreadHandle = usize;

    #[link(name = "pthread")]
    extern "system" {

        #[link_name = "pthread_key_delete"]
        pub fn tls_free(key: u32) -> i32;

        #[link_name = "pthread_getspecific"]
        pub fn tls_get(key: u32) -> usize;

        #[link_name = "pthread_setspecific"]
        pub fn tls_set(key: u32, value: usize) -> i32;

        #[link_name = "sched_yield"]
        pub fn thread_yield() -> i32;

        #[link_name = "pthread_self"]
        pub fn thread_current() -> ThreadHandle;

        fn pthread_exit(_: usize);
        fn pthread_key_create(key: *mut u32, destructor: usize);
        fn pthread_create(id: *mut ThreadHandle, _: usize, func: usize, arg: usize) -> i32;
    }

    #[inline]
    pub unsafe fn thread_exit() {
        pthread_exit(0);
    }

    #[inline]
    pub unsafe fn tls_alloc() -> u32 {
        let mut key = 0;
        pthread_key_create(&mut key, 0);
        key
    }

    #[inline]
    pub unsafe fn thread_create(func: usize, arg: usize) -> ThreadHandle {
        let mut handle = 0;
        pthread_create(&mut handle, 0, func, arg);
        handle
    }
}

#[cfg(windows)]
mod threading {
    use super::super::{File, Handle};
    pub type ThreadHandle = File;

    extern "system" {
        
        #[link_name = "TlsAlloc"]
        pub fn tls_alloc() -> u32;

        #[link_name = "TlsFree"]
        pub fn tls_free(index: u32) -> bool;

        #[link_name = "TlsGetValue"]
        pub fn tls_get(index: u32) -> usize;

        #[link_name = "TlsSetValue"]
        pub fn tls_set(index: u32, value: usize) -> bool;

        #[link_name = "SwitchToThread"]
        pub fn thread_yield() -> bool;

        fn ExitThread(_: u32);
        fn GetCurrentThread() -> Handle;
        fn CreateThread(_: usize, stack: usize, func: usize, arg: usize, flags: i32, id: *mut Handle) -> usize;
    }

    #[inline]
    pub unsafe fn thread_exit() {
        ExitThread(0);
    }

    #[inline]
    pub unsafe fn thread_current() -> ThreadHandle {
        GetCurrentThread().into()
    }

    #[inline]
    pub unsafe fn thread_create(func: usize, arg: usize) -> ThreadHandle {
        let mut handle = 0;
        CreateThread(0, 0, func, arg, 0, &mut handle);
        handle.into()
    }
}