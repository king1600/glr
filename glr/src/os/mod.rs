pub use self::ffi::*;
pub use core::intrinsics::*;

#[cfg(unix)]
pub mod ffi {
    pub use libc::*;
}

#[cfg(windows)]
pub mod ffi {
    pub use winapi::ctypes::*;
    pub use winapi::um::winnt::*;
    pub use winapi::um::memoryapi::*;
    pub use winapi::um::handleapi::*;
}

pub trait CString {
    fn c_str(&self) -> *mut c_char;
}

impl CString for str {
    fn c_str(&self) -> *mut c_char {
        self.as_ptr() as *mut c_char
    }
}

#[allow(dead_code)] pub mod mem;