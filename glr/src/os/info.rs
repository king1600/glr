use super::*;

pub struct SysInfo {
    pub num_cpus: usize,
    pub page_size: usize,
    pub huge_page_size: usize,
}

lazy_static! {
    pub static ref SYS_INFO: SysInfo = unsafe { get_system_info() };
}

#[cfg(windows)]
unsafe fn get_system_info() -> SysInfo {
    let mut info = core::mem::uninitialized();
    GetSystemInfo(&mut info);

    SysInfo {
        page_size: info.dwPageSize as usize,
        num_cpus: info.dwNumberOfProcessors as usize,
        huge_page_size: GetLargePageMinimum(),
    }
}

#[cfg(unix)]
unsafe fn get_system_info() -> SysInfo {
    let mut huge_page_size = 0;
    let page_size = sysconf(_SC_PAGESIZE) as usize;
    let num_cpus = sysconf(_SC_NPROCESSORS_CONF) as usize;
    
    let file = fopen("/proc/meminfo\0".c_str(), "r\0".c_str());
    if !file.is_null() {
        let mut line = [0; 128];
        let size = (line.len() - 1) as i32;
        while !fgets(line.as_mut_ptr(), size, file).is_null() {
            if !strstr(line.as_ptr(), "Hugepagesize:\0".c_str()).is_null() {
                huge_page_size = (strtol((&line[13..]).as_ptr(), null_mut(), 10) * 1024) as usize;
            }
        }
    }

    SysInfo {
        num_cpus,
        page_size,
        huge_page_size
    }
}

