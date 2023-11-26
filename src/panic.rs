//! This module provides a panic handler, which is required for `#![no_std]` binaries.

use crate::io::errno;
use core::{
    fmt::{self, Write},
    panic::PanicInfo,
};
use libc::c_void;

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut stderr = StderrIgnoringErrors;
    let _ = write!(stderr, "{}", info);
    // SAFETY: This function is actually safe...
    unsafe {
        libc::abort();
    }
}

/// A `Write` that writes to standard error, ignoring errors that occur along the way.
///
/// We can't rely on eprint!() here, since that panics when writing to stderr gets an error.
struct StderrIgnoringErrors;

impl Write for StderrIgnoringErrors {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut bs = s.as_bytes();
        while !bs.is_empty() {
            let ptr = bs.as_ptr() as *const c_void;
            let len = bs.len();
            // SAFETY: We're writing exactly the valid range of bs.
            let ret = unsafe { libc::write(2, ptr, len) };
            if ret < 0 {
                // If we got an EINTR, repeat the call.
                if errno() == libc::EINTR {
                    continue;
                } else {
                    // Otherwise, ignore the error, but stop printing.
                    return Ok(());
                }
            }

            // Advance bs by ret.
            let ret = ret as usize;
            if ret > bs.len() {
                // We can't panic here, no matter how much we might want to...
                return Ok(());
            }
            bs = &bs[ret..];
        }
        Ok(())
    }
}
