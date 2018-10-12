use crate::os::*;
use libc::printf;
use core::intrinsics::abort;
use core::fmt::{write, Write, Result};

struct Output;

impl Write for Output {
    fn write_str(&mut self, s: &str) -> Result {
        printf("%*.*s\0".c_str(), s.len(), s.len(), s.c_str());
        Ok(())
    }
}

#[no_mangle]
#[lang = "eh_personality"]
pub extern fn rust_eh_personality() {}

#[no_mangle]
#[lang = "eh_unwind_resume"]
pub extern fn rust_eh_unwind_resume() {}

#[no_mangle]
#[lang = "panic_impl"]
pub extern fn rust_begin_panic(info: &core::panic::PanicInfo) -> ! {
    let mut output = Output {};
    unsafe {
        output.write_str("[GLR] Panic");
        if let Some(loc) = info.location() {
            write!(output, " at {}:{}", loc.file(), loc.line());
        }
        if let Some(&message) = info.message() {
            write(&mut Output::from(stderr), message);
        }
        abort()
    }
}