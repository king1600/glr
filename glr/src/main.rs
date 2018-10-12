#![no_std]
#![no_main]
#![feature(const_fn)]
#![feature(start, lang_items)]
#![feature(asm, core_intrinsics)]

extern crate libc;
#[cfg(windows)] extern crate winapi;

#[allow(dead_code)] pub mod os;
#[allow(dead_code)] pub mod panic;

use self::panic::*;

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}