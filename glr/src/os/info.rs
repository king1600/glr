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

}

