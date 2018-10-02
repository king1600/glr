#![no_std]
#![no_main]

extern crate libc;
extern crate kernel32;

#[allow(dead_code)] pub mod os;

#[no_mangle]
pub unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}