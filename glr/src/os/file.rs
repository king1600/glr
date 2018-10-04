use self::files::*;

#[cfg(linux)] pub type Handle = i32;
#[cfg(windows)] pub type Handle = usize;

pub struct File {
    handle: Handle,
}

impl File {
    #[inline(always)]
    pub const fn invalid() -> Handle {
        -1isize as Handle
    }
}

impl From<Handle> for File {
    fn from(handle: Handle) -> File {
        File { handle }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.handle > 2 {
            unsafe { file_close(self.handle); }
        }
    }
}

#[cfg(linux)]
pub mod files {
    use super::Handle;

    extern "system" {
        #[link_name = "close"]
        pub fn file_close(fd: Handle);
    }
}

#[cfg(windows)]
pub mod files {
    use super::Handle;

    extern "system" {
        #[link_name = "CloseHandle"]
        pub fn file_close(handle: Handle) -> bool;
    }
}