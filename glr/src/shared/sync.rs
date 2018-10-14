use super::*;
use self::inner_impl::*;

pub struct CondVar {
    handle: CondHandle,
}

pub struct Mutex<T> {
    value: T,
    handle: MutexHandle,
}

pub struct Lock<'a, T> {
    pub value: &'a T,
    handle: &'a MutexHandle,
}

impl<'a, T> core::ops::Drop for Lock<'a, T> {
    fn drop(&mut self) {
        self.unlock();
    }
}

impl<'a, T> core::ops::Deref for Lock<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> core::ops::DerefMut for Lock<'a, T> {
    type Target = T;
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.value as *const _ as *mut _) }
    }
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            handle: unsafe { Self::create() },
        }
    }

    #[inline]
    pub fn lock<'a>(&'a self) -> Lock<'a, T> {
        Lock::new(&self.value, &self.handle)
    }

    #[inline]
    pub fn try_lock<'a>(&'a self) -> Option<Lock<'a, T>> {
        Lock::try_new(&self.value, &self.handle)
    }

    #[inline(always)]
    unsafe fn create() -> MutexHandle {
        #[cfg(unix)] {
            PTHREAD_MUTEX_INITIALIZER
        } #[cfg(windows)] {
            let mut mutex = core::mem::uninitialized();
            InitializeCriticalSection(&mut mutex);
            mutex
        }
    }
}

#[cfg(unix)]
mod inner_impl {
    use super::*;
    pub type CondHandle = pthread_cond_t;
    pub type MutexHandle = pthread_mutex_t;

    impl CondVar {
        #[inline]
        pub fn new() -> Self {
            Self { handle: PTHREAD_COND_INITIALIZER }
        }

        #[inline]
        pub fn signal(&self) {
            unsafe { pthread_cond_signal(&self.handle as *const _ as *mut _); }
        }
        
        #[inline]
        pub fn wait(&self, mutex: &MutexHandle) {
            unsafe { pthread_cond_wait(&self.handle as *const _ as *mut _, mutex as *mut _); }
        }
    }

    impl<'a, T> Lock<'a, T> {
        #[inline]
        pub fn unlock(&self) {
            unsafe { pthread_mutex_unlock(&self.handle as *const _ as *mut _); }
        }
        
        #[inline]
        pub fn new(value: &'a T, handle: &'a MutexHandle) -> Self {
            unsafe { pthread_mutex_lock(&handle as *const _ as *mut _); }
            Self { value, handle }
        }

        #[inline]
        pub fn try_new(value: &'a T, handle: &'a MutexHandle) -> Option<Self> {
            if unsafe { pthread_mutex_trylock(&handle as *const _ as *mut _) == 0 } {
                Some(Self { value, handle })
            } else {
                None
            }
        }
    }
}

#[cfg(windows)]
mod inner_impl {
    use super::*;
    pub type CondHandle = CONDITION_VARIABLE;
    pub type MutexHandle = CRITICAL_SECTION;

    impl CondVar {
        #[inline]
        pub fn new() -> Self {
            unsafe {
                let mut handle = core::mem::uninitialized();
                InitializeConditionVariable(&mut handle);
                Self { handle }
            }
        }

        #[inline]
        pub fn signal(&self) {
            unsafe { WakeConditionVariable(&self.handle as *const _ as *mut _); }
        }
        
        #[inline]
        pub fn wait(&self, mutex: &MutexHandle) {
            unsafe {
                SleepConditionVariableCS(
                    &self.handle as *const _ as *mut _,
                    mutex as *const _ as *mut _,
                    INFINITE);
            }
        }
    }

    impl<'a, T> Lock<'a, T> {
        #[inline]
        pub fn unlock(&self) {
            unsafe { LeaveCriticalSection(&self.handle as *const _ as *mut _); }
        }
        
        #[inline]
        pub fn new(value: &'a T, handle: &'a MutexHandle) -> Self {
            unsafe { EnterCriticalSection(&handle as *const _ as *mut _); }
            Self { value, handle }
        }

        #[inline]
        pub fn try_new(value: &'a T, handle: &'a MutexHandle) -> Option<Self> {
            if unsafe { TryEnterCriticalSection(&handle as *const _ as *mut _) == 1 } {
                Some(Self { value, handle })
            } else {
                None
            }
        }
    }
}