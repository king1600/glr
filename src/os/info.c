#include "info.h"

static glr_sysinfo_t __glr_sys_info;

glr_sysinfo_t* glr_sys_info() {
    return &__glr_sys_info;
}

#ifdef GLR_WINDOWS
    void glr_sys_info_init() {
        SYSTEM_INFO info;
        GetSystemInfo(&info);
        
        __glr_sys_info.page_size = info.dwPageSize;
        __glr_sys_info.num_cpus = info.dwNumberOfProcessors;
        __glr_sys_info.huge_page_size = GetLargePageMinimum();    
    }

#else
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
    #include <unistd.h>
    #include <sys/sysinfo.h>

    void glr_sys_info_init() {
        __glr_sys_info.huge_page_size = 0;
        __glr_sys_info.num_cpus = get_nprocs();
        __glr_sys_info.page_size = getpagesize();

        FILE* proc;
        char line[0xff] = { 0 };
        if ((proc = fopen("/proc/meminfo", "r"))) {
            while (fgets(line, sizeof(line) - 1, proc))
                if (strstr(line, "Hugepagesize:"))
                    __glr_sys_info.huge_page_size = strtol(line + 13, 0, 10) * 1024;
            fclose(proc);
        }
    }
#endif