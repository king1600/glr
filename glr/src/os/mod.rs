pub use self::ffi::*;
pub use core::intrinsics::*;

pub const NULL: *mut c_void = 0 as *mut _;

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
    pub use winapi::um::sysinfoapi::*;
    pub use winapi::shared::minwindef::*;
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
#[allow(dead_code)] pub mod info;