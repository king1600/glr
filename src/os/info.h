#ifndef _GLR_INFO_H
#define _GLR_INFO_H

#include "sys.h"

typedef struct {
    size_t num_cpus;
    size_t page_size;
    size_t huge_page_size;
} glr_sysinfo_t;

void glr_sys_info_init();

glr_sysinfo_t* glr_sys_info();

#endif // _GLR_INFO_H