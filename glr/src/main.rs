#![no_std]
#![no_main]
#![feature(const_fn)]
#![feature(asm, core_intrinsics)]

extern crate libc;
#[cfg(windows)] extern crate winapi;
#[macro_use] extern crate lazy_static;

#[allow(dead_code)] pub mod os;

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}