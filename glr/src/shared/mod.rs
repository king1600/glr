pub use self::ffi::*;
pub use core::ptr::null_mut;
pub use core::intrinsics::*;

pub const NULL: *mut c_void = 0 as *mut c_void;

#[macro_use]
#[allow(dead_code)]
pub mod print;

#[allow(dead_code)]
pub mod sync;

#[allow(dead_code)]
pub mod mem;

#[cfg(unix)]
pub mod ffi {
    pub use libc::*;
}

#[cfg(windows)]
pub mod ffi {
    pub use winapi::ctypes::*;
    pub use winapi::shared::minwindef::*;

    pub use winapi::um::winnt::*;
    pub use winapi::um::winbase::*;
    pub use winapi::um::fileapi::*;
    pub use winapi::um::synchapi::*;
    pub use winapi::um::debugapi::*;
    pub use winapi::um::memoryapi::*;
    pub use winapi::um::handleapi::*;
    pub use winapi::um::sysinfoapi::*;
    pub use winapi::um::minwinbase::*;
    pub use winapi::um::processthreadsapi::*;    
}

pub trait CString {
    fn c_str(&self) -> *mut c_char;
}

impl CString for str {
    fn c_str(&self) -> *mut c_char {
        self.as_ptr() as *mut c_char
    }
}