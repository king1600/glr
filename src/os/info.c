#include "info.h"


#ifdef GLR_WINDOWS
    void glr_sys_info_init() {
        SYSTEM_INFO info;
        GetSystemInfo(&info);
        
        GLR_SYS_INFO.page_size = info.dwPageSize;
        GLR_SYS_INFO.num_cpus = info.dwNumberOfProcessors;
        GLR_SYS_INFO.huge_page_size = GetLargePageMinimum();    
    }

#else
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
    #include <unistd.h>
    #include <sys/sysinfo.h>

    void glr_sys_info_init() {
        GLR_SYS_INFO.huge_page_size = 0;
        GLR_SYS_INFO.num_cpus = get_nprocs();
        GLR_SYS_INFO.page_size = getpagesize();

        FILE* proc;
        char line[0xff] = { 0 };
        if ((proc = fopen("/proc/meminfo", "r")))
            while (fgets(line, sizeof(line) -1, proc))
                if (strstr(line, "Hugepagesize:"))
                    GLR_SYS_INFO.huge_page_size = strtol(line + 13, 0, 10) * 1024;
    }
#endif