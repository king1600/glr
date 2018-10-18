#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(panic_info_message)]

#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[allow(dead_code)]
pub mod shared;
#[allow(dead_code)]
pub mod panic;
#[allow(dead_code)]
pub mod bytecode;

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    println!("Hello world");
    0
}