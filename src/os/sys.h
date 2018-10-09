#ifndef _GLR_SYS_H
#define _GLR_SYS_H

#include "../glr.h"
#include <assert.h>

#if defined(_WIN32)
    #define GLR_WINDOWS
    #include "Windows.h"
#else
    #define _GNU_SOURCE
#endif

#if defined(__x86__) || defined(__x86_64__)
    #define GLR_x86
#endif
#if defined(__x86_64__) || defined(__LP64__)
    #define GLR_64
#endif

#define GLR_THREAD_LOCAL __thread
#define GLR_LIKELY(expr) __builtin_expect(!!(expr), 1)
#define GLR_UNLIKELY(expr) __builtin_expect(!!(expr), 0)
#define GLR_FORCE_INLINE inline __attribute__((__always_inline__))
#define GLR_ASSERT(expr, message) assert(expr && message)
#define GLR_ALIGN_TO(size, align) ((((align)-1) & (size)) ? (((size)+(align)) & ~((size)-1)) : (size))

#endif // _GLR_SYS_H
