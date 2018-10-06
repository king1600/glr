#ifndef _GLR_INFO_H
#define _GLR_INFO_H

#include "sys.h"

typedef struct {
    size_t num_cpus;
    size_t page_size;
    size_t huge_page_size;
} glr_sysinfo_t;

static glr_sysinfo_t GLR_SYS_INFO;

void glr_sys_info_init();

#endif // _GLR_INFO_H