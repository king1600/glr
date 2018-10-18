pub use self::ffi::*;
pub use core::ptr::null_mut;
pub use core::intrinsics::*;

pub const NULL: *mut c_void = 0 as *mut c_void;

pub trait CString {
    fn c_str(&self) -> *mut c_char;
}

impl CString for str {
    fn c_str(&self) -> *mut c_char {
        self.as_ptr() as *mut c_char
    }
}

#[cfg(unix)]
pub mod ffi {
    pub use libc::*;
}


#[cfg(windows)]
pub mod ffi {
    pub use winapi::ctypes::*;
    pub use winapi::shared::minwindef::*;
    
    pub use winapi::um::winnt::*;
    pub use winapi::um::memoryapi::*;
    pub use winapi::um::sysinfoapi::*;

    extern "C" {
        pub fn printf(format: *const c_char, ...) -> i32;
    }
}