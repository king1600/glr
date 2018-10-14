#![no_std]
#![no_main]
#![feature(const_fn, try_blocks)]
#![feature(asm, core_intrinsics)]

extern crate libc;
#[cfg(windows)] extern crate winapi;
#[macro_use] extern crate lazy_static;

#[macro_use]
#[allow(dead_code)]
pub mod shared;

#[allow(dead_code)]
pub mod vm;

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    print_err!("[GLR Panic] Panic");

    match info.location() {
        None      => print_err!(" at ??:??"),
        Some(loc) => print_err!(" at {}:{}", loc.file(), loc.line()),
    }

    unsafe { core::intrinsics::abort() }
}