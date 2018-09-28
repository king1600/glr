#![no_std]
#![no_main]
#![feature(start)]
#![feature(lang_items)]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

extern crate libc;

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}

#[lang = "panic_impl"]
#[no_mangle]
pub extern fn rust_begin_panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}