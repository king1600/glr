use super::*;

pub struct SysInfo {
    num_cpus: usize,
    page_size: usize,
    huge_page_size: usize,
}

lazy_static! {
    static ref INNER_SYSTEM_INFO: SysInfo = unsafe { get_system_info() };
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
    let num_cpus = get_nprocs() as usize;
    let page_size = getpagesize() as usize;

    let file = fopen("/proc/meminfo\0".c_str(), "r\0".c_str());
    if !file.is_null() {
        let mut line = [0; 128];
        while !fgets(line.as_mut_ptr(), line.len() - 1, file).is_null() {
            if !strstr(line, "Hugepagesize:\0".c_str()).is_null() {
                huge_page_size = (strtol((&line[13..]).as_ptr(), 0, 10) * 1024) as usize;
            }
        }
    }

    SysInfo {
        num_cpus,
        page_size,
        huge_page_size
    }
}

