use super::*;
use core::mem::uninitialized;
pub use core::sync::atomic::*;

pub struct Mutex {
    #[cfg(unix)] lock: pthread_mutex_t,
    #[cfg(windows)] lock: CRITICAL_SECTION,
}

pub struct CondVar {
    #[cfg(unix)] cond: pthread_cond_t,
    #[cfg(windows)] cond: CONDITION_VARIABLE,
}

impl CondVar {
    #[inline]
    pub fn new() -> Self {
        Self { cond: unsafe { inner_impl::cond_create() } }
    }

    #[inline]
    pub fn signal(&self) {
        unsafe { inner_impl::cond_signal(&self.cond); }
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        unsafe { inner_impl::cond_wait(&self.cond, &mutex.lock); }
    }
}

impl Mutex {
    #[inline]
    pub fn new() -> Self {
        Self { lock: unsafe { inner_impl::mutex_create() } }
    }

    #[inline]
    pub fn lock(&self) {
        unsafe { inner_impl::mutex_lock(&self.lock); }
    }

    #[inline]
    pub fn unlock(&self) {
        unsafe { inner_impl::mutex_unlock(&self.lock); }
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        unsafe { inner_impl::mutex_trylock(&self.lock) }
    }
}

#[cfg(unix)]
mod inner_impl {
    use super::*;

    #[inline]
    pub unsafe fn mutex_create() -> pthread_mutex_t {
        PTHREAD_MUTEX_INITIALIZER
    }

    #[inline]
    pub unsafe fn mutex_lock(lock: *const pthread_mutex_t) {
        pthread_mutex_lock(lock as *mut _);
    }

    #[inline]
    pub unsafe fn mutex_unlock(lock: *const pthread_mutex_t) {
        pthread_mutex_unlock(lock as *mut _);
    }

    #[inline]
    pub unsafe fn mutex_trylock(lock: *const pthread_mutex_t) -> bool {
        pthread_mutex_trylock(lock as *mut _) == 0;
    }

    #[inline]
    pub unsafe fn cond_create() -> pthread_cond_t {
        PTHREAD_COND_INITIALIZER
    }

    #[inline]
    pub unsafe fn cond_signal(cond: *const pthread_cond_t) {
        pthread_cond_signal(cond as *mut _);
    }

    #[inline]
    pub unsafe fn cond_wait(cond: *const pthread_cond_t, mutex: *const pthread_mutex_t) {
        pthread_cond_wait(cond as *mut _, mutex as *mut _);
    }
}

#[cfg(windows)]
mod inner_impl {
    use super::*;

    #[inline]
    pub unsafe fn mutex_create() -> CRITICAL_SECTION {
        let mut mutex = uninitialized();
        InitializeCriticalSection(&mut mutex);
        mutex
    }

    #[inline]
    pub unsafe fn mutex_lock(lock: *const CRITICAL_SECTION) {
        EnterCriticalSection(lock as *mut _);
    }

    #[inline]
    pub unsafe fn mutex_unlock(lock: *const CRITICAL_SECTION) {
        LeaveCriticalSection(lock as *mut _);
    }

    #[inline]
    pub unsafe fn mutex_trylock(lock: *const CRITICAL_SECTION) -> bool {
        TryEnterCriticalSection(lock as *mut _) == 1
    }

    #[inline]
    pub unsafe fn cond_create() -> CONDITION_VARIABLE {
        let mut cond = uninitialized();
        InitializeConditionVariable(&mut cond);
        cond
    }

    #[inline]
    pub unsafe fn cond_signal(cond: *const CONDITION_VARIABLE) {
        WakeConditionVariable(cond as *mut _);
    }

    #[inline]
    pub unsafe fn cond_wait(cond: *const CONDITION_VARIABLE, mutex: *const CRITICAL_SECTION) {
        SleepConditionVariableCS(cond as *mut _, mutex as *mut _, INFINITE);
    }
}