#![no_std]
#![no_main]
#![feature(const_fn)]

// crate imports
extern crate panic_abort;

// modules
#[allow(dead_code)] pub mod os;
#[allow(dead_code)] pub mod gc;

#[no_mangle]
pub unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {

    #[cfg(not(target_pointer_width = "64"))]
    compile_error!("ZGC requires 64bit pointers so platform is not supported");

    #[cfg(not(target_arch = "x86_64"))]
    compile_error!("JIT currently only supports x86_64");

    #[cfg(not(any(linux, windows)))]
    compile_error!("GLR only supports windows and linux apis");

    0
}