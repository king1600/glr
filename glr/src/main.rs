#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(try_blocks)]
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
    #[cfg(not(target_arch = "x86_64"))]
    compile_error!("GLR only supports x86_64");
    
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    compile_error!("GLR only supports windows and linux");

    let x = unsafe { bytecode::interpreter::interpret() };
    println!("Hello world {}", x);
    0
}