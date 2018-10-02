
pub struct File {
    pub(super) fd: i32,
}

impl From<i32> for File {
    fn from(fd: i32) -> File {
        File { fd }
    }
}

impl Drop for File {
    #[cfg(not(windows))]
    fn drop(&mut self) {
        unsafe { libc::close(self.fd); }
    }

    #[cfg(windows)]
    fn drop(&mut self) {
        unsafe { kernel32::CloseHandle(self.fd as kernel32::HANDLE); }
    }
}
