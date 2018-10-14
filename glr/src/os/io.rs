use super::*;
use super::sync::Mutex;
use super::mem::Handle;
use super::pool::{offsets, PoolAllocator};

use core::mem::uninitialized;
use core::fmt::{write, Write, Result, Arguments};
use core::intrinsics::copy_nonoverlapping as memcpy;

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => (crate::os::io::print_stdout(format_args!($($arg)*)));
}

macro_rules! print_err {
    ($($arg:tt)*) => (crate::os::io::print_stderr(format_args!($($arg)*)));
}

struct TTYPort {
    mutex: Mutex,
    handle: Handle,
    buffer: Option<PoolAllocator>,
}

unsafe impl Send for TTYPort {}
unsafe impl Sync for TTYPort {}

impl TTYPort {
    pub fn new(handle: Handle, address_range: usize) -> Self {
        Self {
            handle: handle,
            mutex: Mutex::new(),
            buffer: PoolAllocator::alloc_at(address_range),
        }
    }

    #[inline]
    pub fn borrow_mut(&self) -> &mut Self {
        unsafe { &mut *(self as *const _ as *mut _) }
    } 
}

#[inline(always)]
pub fn print_stdout(args: Arguments) {
    let _ = write(STD_PORTS.0.borrow_mut(), args);
}

#[inline(always)]
pub fn print_stderr(args: Arguments) {
    let _ = write(STD_PORTS.1.borrow_mut(), args);
}

lazy_static! {
    static ref STD_PORTS: (TTYPort, TTYPort) = {
        let (stdout, stderr, _) = unsafe { std_impl::get_std_handles() };
        let stdout = TTYPort::new(stdout, offsets::STDOUT_BUFFER);
        let stderr = TTYPort::new(stderr, offsets::STDERR_BUFFER);
        (stdout, stderr)
    };
}

impl Write for TTYPort {
    fn write_str(&mut self, string: &str) -> Result {
        unsafe {
            let mut flush = string.contains('\n');
            let string = string.as_bytes();
            self.mutex.lock();

            match &mut self.buffer {
                None => flush = true,
                Some(buffer) => match buffer.alloc_bytes(string.len()) {
                    Some(bytes) => memcpy(string.as_ptr(), bytes, string.len()),
                    None => {
                        std_impl::write(self.handle, buffer.taken_bytes());
                        flush = flush || string.len() > buffer.len();
                        buffer.reset();
                    }
                },
            }

            if flush {
                std_impl::write(self.handle, string);
            }

            self.mutex.unlock();
            Ok(())
        }
    }
}

#[cfg(unix)]
mod std_impl {
    use super::*;

    #[inline]
    pub unsafe fn write(handle: Handle, bytes: &[u8]) {
        libc::write(handle, bytes.as_ptr() as *mut _, bytes.len());
    }

    #[inline]
    pub unsafe fn get_std_handles() -> (Handle, Handle, Handle) {
        (libc::STDOUT_FILENO, libc::STDERR_FILENO, libc::STDIN_FILENO)
    }
}

#[cfg(windows)]
mod std_impl {
    use super::*;

    #[inline]
    pub unsafe fn write(handle: Handle, bytes: &[u8]) {
        let mut written = 0;
        WriteFile(handle, bytes.as_ptr() as *const _, bytes.len() as u32, &mut written, null_mut());
    }

    #[inline]
    pub unsafe fn get_std_handles() -> (Handle, Handle, Handle) {
        let mut info: STARTUPINFOW = uninitialized();
        GetStartupInfoW(&mut info);
        (info.hStdOutput, info.hStdError, info.hStdInput)
    }
}