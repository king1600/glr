#ifndef _GLR_ATOMIC_H
#define _GLR_ATOMIC_H

#include <stdbool.h>
#include <stdatomic.h>

#define GLR_ATOMIC(T) volatile T
#define GLR_ALIGN(N) __attribute__((aligned(N)))

#define GLR_ATOMIC_RELAXED __ATOMIC_RELAXED
#define GLR_ATOMIC_CONSUME __ATOMIC_CONSUME
#define GLR_ATOMIC_ACQUIRE __ATOMIC_ACQUIRE
#define GLR_ATOMIC_RELEASE __ATOMIC_RELEASE
#define GLR_ATOMIC_ACQ_REL __ATOMIC_ACQ_REL
#define GLR_ATOMIC_SEQ_CST __ATOMIC_SEQ_CST

#define glr_atomic_fence(memory_order) \
    __atomic_thread_fence(memory_order)

#define glr_atomic_load(ptr, memory_order) \
    __atomic_load_n(ptr, memory_order)

#define glr_atomic_store(ptr, value, memory_order) \
    __atomic_store_n(ptr, value, memory_order)

#define glr_atomic_add(ptr, value, memory_order) \
    __atomic_fetch_add(ptr, value, memory_order)

#define glr_atomic_sub(ptr, value, memory_order) \
    __atomic_fetch_sub(ptr, value, memory_order)

#define glr_atomic_swap(ptr, value, memory_order) \
    __atomic_exchange_n(ptr, value, memory_order)

#define glr_atomic_cas_weak(ptr, expect, swap, memory_order) \
    __atomic_compare_exchange_n(ptr, expect, swap, true, memory_order, memory_order)
    
#define glr_atomic_cas_strong(ptr, expect, swap, memory_order) \
    __atomic_compare_exchange_n(ptr, expect, swap, false, memory_order, memory_order)

#endif // _GLR_ATOMIC_H