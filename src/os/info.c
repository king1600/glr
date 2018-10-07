#include "info.h"

#ifdef GLR_WINDOWS
    void glr_sys_info_init(glr_sysinfo_t* info) {
        SYSTEM_INFO sys_info;
        GetSystemInfo(&sys_info);
        
        info->page_size = sys_info.dwPageSize;
        info->num_cpus = sys_info.dwNumberOfProcessors;
        info->huge_page_size = GetLargePageMinimum();    
    }

#else
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
    #include <unistd.h>
    #include <sys/sysinfo.h>

    void glr_sys_info_init(glr_sysinfo_t* info) {
        info->huge_page_size = 0;
        info->num_cpus = get_nprocs();
        info->page_size = getpagesize();

        FILE* proc;
        char line[0xff] = { 0 };
        if ((proc = fopen("/proc/meminfo", "r"))) {
            while (fgets(line, sizeof(line) - 1, proc))
                if (strstr(line, "Hugepagesize:"))
                    info->huge_page_size = strtol(line + 13, 0, 10) * 1024;
            fclose(proc);
        }
    }
#endif