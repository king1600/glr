#ifndef _GLR_HANDLE_H
#define _GLR_HANDLE_H

#include "sys.h"

#ifdef GLR_WINDOWS

    #define GLR_FD HANDLE
    #define GLR_BAD_FD INVALID_HANDLE_VALUE

    #define glr_fd_close(fd) CloseHandle(fd)

#else
    #include <unistd.h>
    
    #define GLR_FD int
    #define GLR_BAD_FD -1

    #define glr_fd_close(fd) close(fd)

#endif

#endif // _GLR_HANDLE_H