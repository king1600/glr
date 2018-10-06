#ifndef _GLR_THREAD_H
#define _GLR_THREAD_H

#include "sys.h"

typedef void* (*glr_thread_func_t) (void* argv);

#ifdef GLR_WINDOWS
    typedef HANDLE glr_thread_t;
    typedef SRWLOCK glr_rwlock_t;
    typedef CRITICAL_SECTION glr_mutex_t;
    typedef CONDITION_VARIABLE glr_condvar_t;

    // condition variable interface
    #define glr_condvar_free(cv)
    #define glr_condvar_init(cv) InitializeConditionVariable(cv)
    #define glr_condvar_signal(cv) WakeConditionVariable(cv)
    #define glr_condvar_wait(cv, mutex) SleepConditionVariableCS(cv, mutex, INFINITE)

    // mutex interface
    #define glr_mutex_free(mutex)
    #define glr_mutex_init(mutex) InitializeCriticalSection(mutex)
    #define glr_mutex_lock(mutex) EnterCriticalSection(mutex)
    #define glr_mutex_unlock(mutex) LeaveCriticalSection(mutex)

    // read-write lock interface
    #define glr_rwlock_free(rw)
    #define glr_rwlock_init(rw) InitializeSRWLock(rw)
    #define glr_rwlock_rlock(rw) AcquireSRWLockShared(rw)
    #define glr_rwlock_wlock(rw) AcquireSRWLockExclusive(rw)
    #define glr_rwlock_runlock(rw) ReleaseSRWLockShared(rw)
    #define glr_rwlock_wunlock(rw) ReleaseSRWLockExclusive(rw)
    #define glr_rwlock_tryrlock(rw) TryAcquireSRWLockShared(rw)
    #define glr_rwlock_trywlock(rw) TryAcquireSRWLockExclusive(rw)

    // thread interface
    #define glr_thread_exit() ExitThread(NULL)
    #define glr_thread_yield() SwitchToThread()
    #define GLR_THREAD_FUNC(name, argv) DWORD WINAPI name(PVOID argv)

    GLR_FORCE_INLINE void glr_thread_join(glr_thread_t thread) {
        WaitForSingleObject(thread, INFINITE);
        CloseHandle(thread);
    }

    GLR_FORCE_INLINE glr_thread_t glr_thread_init(glr_thread_func_t func, void* argv) {
        DWORD thread_id;
        return CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)func, (LPVOID) argv, 0, &thread_id);
    }

#else
    #include <sched.h>
    #include <pthread.h>

    typedef pthread_t glr_thread_t;
    typedef pthread_mutex_t glr_mutex_t;
    typedef pthread_cond_t glr_condvar_t;
    typedef pthread_rwlock_t glr_rwlock_t;

    // condition variable interface
    #define glr_condvar_free(cv) pthread_cond_destroy(cv)
    #define glr_condvar_init(cv) pthread_cond_init(cv, NULL)
    #define glr_condvar_signal(cv) pthread_cond_signal(cv)
    #define glr_condvar_wait(cv, mutex) pthread_cond_wait(cv, mutex)

    // mutex interface
    #define glr_mutex_free(mutex) pthread_mutex_destroy(mutex)
    #define glr_mutex_init(mutex) pthread_mutex_init(mutex)
    #define glr_mutex_lock(mutex) pthread_mutex_lock(mutex)
    #define glr_mutex_trylock(mutex) pthread_mutex_trylock(mutex)
    #define glr_mutex_unlock(mutex) pthread_mutex_unlock(mutex)

    // read-write lock interface
    #define glr_rwlock_free(rw) pthread_rwlock_destroy(rw)
    #define glr_rwlock_init(rw) pthread_rwlock_init(rw)
    #define glr_rwlock_rlock(rw) pthread_rwlock_rdlock(rw)
    #define glr_rwlock_wlock(rw) pthread_rwlock_wrlock(rw)
    #define glr_rwlock_runlock(rw) pthread_rwlock_unlock(rw)
    #define glr_rwlock_wunlock(rw) pthread_rwlock_unlock(rw)
    #define glr_rwlock_tryrlock(rw) pthread_rwlock_tryrdlock(rw)
    #define glr_rwlock_trywlock(rw) pthread_rwlock_trywrlock(rw)

    // thread interface
    #define glr_thread_yield() sched_yield()
    #define glr_thread_exit() pthread_exit(NULL)
    #define glr_thread_join(thread) pthread_join(thread, NULL)
    #define GLR_THREAD_FUNC(name, argv) void* name(void* argv)

    GLR_FORCE_INLINE glr_thread_t glr_thread_init(glr_thread_func_t func, void* argv) {
        glr_thread_t thread;
        pthread_create(&thread, NULL, func, argv);
        return thread;
    }

#endif // GLR_WINDOWS
#endif // _GLR_SYNC_H