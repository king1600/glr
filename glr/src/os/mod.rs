#[allow(dead_code)] pub mod page;
#[allow(dead_code)] pub mod file;

pub use self::{page::*, file::*, types::*};

#[cfg(not(windows))]
pub mod types {
    pub use libc::{c_void, c_char, PT_NULL as NULL};
}

#[cfg(windows)]
pub mod types {
    pub use winapi::{
        ctypes::{c_void, c_char},
        shared::minwindef::DWORD,
        um::winnt::{PVOID as LPVOID}
    };
    pub const NULL: LPVOID = 0 as LPVOID;
}

pub trait CString {
    unsafe fn c_str(&self) -> *mut c_char;
}

impl CString for str {
    unsafe fn c_str(&self) -> *mut c_char {
        self.as_ptr() as *const _ as *mut c_char
    }
}
