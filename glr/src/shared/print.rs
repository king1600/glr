use super::*;
use super::mem::*;
use super::sync::Mutex;

use self::tty_impl::*;
use core::fmt::{self, Write, Result, Arguments};

pub struct StdIo {
    pub stdout: TTYPort,
    pub stderr: TTYPort,
}

pub struct TTYPort {
    handle: Mutex<TTYHandle>,
    buffer: Option<MemoryRange>,
}

#[allow(unused_macros)]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", $($arg)*));
}

#[allow(unused_macros)]
macro_rules! print {
    ($($arg:tt)*) => (crate::shared::print::print_stdout(format_args!($($arg)*)));
}

#[allow(unused_macros)]
macro_rules! print_err {
    ($($arg:tt)*) => (crate::shared::print::print_stderr(format_args!($($arg)*)));
}

unsafe impl Send for TTYPort {}
unsafe impl Sync for TTYPort {}

impl TTYPort {
    #[inline]
    pub fn new(handle: TTYHandle, address_range: usize) -> Self {
        Self {
            handle: Mutex::new(handle),
            buffer: MemoryRange::at(address_range),
        }
    }

    #[inline]
    pub fn borrow_mut(&self) -> &mut Self {
        unsafe { &mut *(self as *const _ as *mut _) }
    }
}

impl Write for TTYPort {
    fn write_str(&mut self, string: &str) -> Result {
        use core::ptr::copy_nonoverlapping as memcpy;
        let handle_lock = self.handle.lock();
        let handle = *handle_lock.value;

        // TODO: Clean this up maybe?
        if let Some(buffer) = &mut self.buffer {
            if let Some(bytes) = buffer.alloc_bytes(string.len()) {
                if string.contains('\n') {
                    tty_write(handle, buffer.drain(), Some(string.as_bytes()));
                } else {
                    unsafe { memcpy(string.as_bytes().as_ptr(), bytes, string.len()) }
                }
            } else {
                tty_write(handle, buffer.drain(), Some(string.as_bytes()));
            }
        } else {
            tty_write(handle, string.as_bytes(), None);
        }

        Ok(())
    }
}

#[inline(always)]
pub fn print_stdout(args: Arguments) {
    let _ = fmt::write(STDIO.stdout.borrow_mut(), args);
}

#[inline(always)]
pub fn print_stderr(args: Arguments) {
    let _ = fmt::write(STDIO.stderr.borrow_mut(), args);
}

lazy_static! {
    static ref STDIO: StdIo = {
        let (stdout, stderr, _) = get_tty_handles();
        let stdout = TTYPort::new(stdout, STDOUT_BUFFER);
        let stderr = TTYPort::new(stderr, STDERR_BUFFER);
        StdIo { stdout, stderr }
    };
}

#[cfg(unix)]
mod tty_impl {
    use super::*;
    pub type TTYHandle = i32;

    #[inline(always)]
    pub fn get_tty_handles() -> (TTYHandle, TTYHandle, TTYHandle) {
        (STDOUT_FILENO, STDERR_FILENO, STDIN_FILENO)
    }

    #[inline]
    pub fn tty_write(handle: TTYHandle, bytes: &[u8], extra: Option<&[u8]>) {
        use core::mem::transmute;
        unsafe {
            if let Some(extra) = extra {
                let iovecs = [transmute(bytes), transmute(extra)];
                writev(handle, &iovecs as *const _, iovecs.len());
            } else {
                write(handle, bytes.as_ptr() as *mut _, bytes.len());
            }
        }
    }
}

#[cfg(windows)]
mod tty_impl {
    use super::*;
    pub type TTYHandle = HANDLE;

    #[inline(always)]
    pub fn get_tty_handles() -> (TTYHandle, TTYHandle, TTYHandle) {
        unsafe {
            let mut info: STARTUPINFOW = core::mem::uninitialized();
            GetStartupInfoW(&mut info);
            (info.hStdOutput, info.hStdError, info.hStdInput)
        }
    }

    #[inline]
    pub fn tty_write(handle: TTYHandle, bytes: &[u8], extra: Option<&[u8]>) {
        unsafe {
            let mut written = 0;
            WriteFile(handle, bytes.as_ptr() as *const _, bytes.len() as u32, &mut written, null_mut());
            // TODO: would be nice to do some sort of scatter io as this could be a hotpath
            if let Some(bytes) = extra {
                 WriteFile(handle, bytes.as_ptr() as *const _, bytes.len() as u32, &mut written, null_mut());
            }
        }
    }
}