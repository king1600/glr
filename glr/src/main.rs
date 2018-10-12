#![no_std]
#![no_main]
#![feature(start, lang_items)]
#![feature(asm, core_intrinsics)]

#[cfg(unix)] extern crate libc;
#[cfg(windows)] extern crate winapi;

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}

#[no_mangle]
#[lang = "eh_personality"]
pub extern fn rust_eh_personality() {}

#[no_mangle]
#[lang = "eh_unwind_resume"]
pub extern fn rust_eh_unwind_resume() {}

#[no_mangle]
#[lang = "panic_impl"]
pub extern fn rust_begin_panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}