#[allow(dead_code)] pub mod page;
#[allow(dead_code)] pub mod file;

pub trait CString {
    unsafe fn c_str(&self) -> *mut libc::c_char;
}

impl CString for str {
    unsafe fn c_str(&self) -> *mut libc::c_char {
        self.as_ptr() as *const _ as *mut libc::c_char
    }
}

