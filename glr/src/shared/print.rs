use super::*;
use core::fmt::{write, Write, Result, Arguments};

pub struct StdOut;

pub static mut STDOUT: StdOut = StdOut {};

#[inline]
pub fn print(args: Arguments) {
    let _ = unsafe { write(&mut STDOUT, args) };
}

impl Write for StdOut {
    fn write_str(&mut self, string: &str) -> Result {
        extern "C" { fn printf(format: *const c_char, ...) -> c_int; }
        let size = string.len() as i32;
        unsafe { printf("%*.*s\0".c_str(), size, size, string.c_str()); }
        Ok(())
    }
}

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}", format_args!($($arg)*)));
}

macro_rules! print {
    ($($arg:tt)*) => (crate::shared::print::print(format_args!($($arg)*)))
}