use super::*;
use core::fmt::{write, Write, Result, Arguments};

macro_rules! println {
    () => (print!("\n"));
    ($($args:tt)*) => (print!("{}\n", format_args!($($args)*)));
}

macro_rules! print {
    ($($args:tt)*) => (crate::shared::print(format_args!($($args)*)));
}

pub fn print(args: Arguments) {
    let _ = write(&mut StdOut, args);
}

pub struct StdOut;

static mut STDOUT: StdOut = StdOut {};

unsafe impl core::marker::Send for StdOut {}
unsafe impl core::marker::Sync for StdOut {}

impl Write for StdOut {
    fn write_str(&mut self, string: &str) -> Result {
        let len = string.len() as i32;
        let data = string.c_str() as *const _;
        unsafe { printf("%*.*s\0".c_str(), len, len, data); }
        Ok(())
    }
}
