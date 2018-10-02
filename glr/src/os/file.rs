pub use self::file_api::*;

pub struct File {
    pub(super) fd: HANDLE,
}

impl Drop for File {
    fn drop(&mut self) {
        file_api::close_file(self.fd);
    }
}

impl From<HANDLE> for File {
    fn from(handle: HANDLE) -> File {
        File { fd: handle }
    }
}

#[cfg(not(windows))]
pub mod file_api {
    use libc::close;
    pub type HANDLE = libc::c_int;
    pub const INVALID_HANDLE: HANDLE = -1;

    #[inline]
    pub fn close_file(fd: HANDLE) {
        close(fd);
    }
}

#[cfg(windows)]
pub mod file_api {
    use winapi::um::{winnt, handleapi::{INVALID_HANDLE_VALUE, CloseHandle}};
    pub type HANDLE = winnt::HANDLE;
    pub const INVALID_HANDLE: HANDLE = INVALID_HANDLE_VALUE;

    #[inline]
    pub fn close_file(fd: HANDLE) {
        unsafe { CloseHandle(fd) };
    }
}
