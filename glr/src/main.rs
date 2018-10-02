#![no_std]
#![no_main]

// crate imports
extern crate libc;
extern crate panic_abort;
#[cfg(windows)] extern crate winapi;

// modules
#[allow(dead_code)] pub mod os;

#[no_mangle]
pub unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {

    #[cfg(not(target_pointer_width = "64"))]
    compile_error!("Only 64bit platforms are supported");
    #[cfg(not(target_arch = "x86_64"))]
    compile_error!("Only x86_64 is currently supported");

    0
}