use core::fmt::{self, Write};
use libc::{__errno_location, c_int, c_void};

/// A `Write` corresponding to a file descriptor.
#[derive(Debug)]
pub struct FdWrite {
    pub fd: i32,
}

impl FdWrite {
    /// Writes bs to the file descriptor, returning errno if there was an error.
    pub fn write_bytes(&mut self, mut bs: &[u8]) -> Result<(), c_int> {
        while !bs.is_empty() {
            let ptr = bs.as_ptr() as *const c_void;
            let len = bs.len();
            // SAFETY: We're writing exactly the valid range of bs.
            let ret = unsafe { libc::write(self.fd, ptr, len) };
            if ret < 0 {
                // If we got an EINTR, repeat the call.
                let err = errno();
                if err == libc::EINTR {
                    continue;
                } else {
                    // Otherwise, return an error.
                    return Err(err);
                }
            }

            // Advance bs by ret.
            let ret = ret as usize;
            bs = &bs[ret..];
        }
        Ok(())
    }
}

impl Write for FdWrite {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

/// Returns a `Write` corresponding to standard output.
pub fn stdout() -> FdWrite {
    FdWrite { fd: 1 }
}

/// Returns a `Write` corresponding to standard error.
pub fn stderr() -> FdWrite {
    FdWrite { fd: 2 }
}

/// Returns the last value of errno.
pub fn errno() -> c_int {
    unsafe {
        // SAFETY: This function doesn't actually have safety preconditions...
        let ptr = __errno_location();

        // SAFETY: __errno_location will always give us a valid thread-unique pointer, so we can
        // dereference it, and we don't even need to worry about a race when doing so.
        *ptr
    }
}

/// Sets errno to zero.
pub fn clear_errno() {
    unsafe {
        // SAFETY: This function doesn't actually have safety preconditions...
        let ptr = __errno_location();

        // SAFETY: __errno_location will always give us a valid thread-unique pointer, so we can
        // mutate through it, and we don't even need to worry about a race when doing so.
        *ptr = 0;
    }
}
