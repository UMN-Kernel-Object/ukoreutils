#![no_std]

use core::ffi::c_void;
use ukoreutils::{die_with_errno, io::errno};

fn main() {
    let mut bs: &[u8] = b"Hello, world!\n";

    while !bs.is_empty() {
        let ptr = bs.as_ptr() as *const c_void;
        let len = bs.len();
        // SAFETY: We're writing exactly the valid range of bs.
        let ret = unsafe { libc::write(1, ptr, len) };
        if ret < 0 {
            // If we got an EINTR, repeat the call.
            let err = errno();
            if err == libc::EINTR {
                continue;
            } else {
                // Otherwise, bail out with the error.
                die_with_errno(err, "write")
            }
        }

        // Advance bs by ret.
        let ret = ret as usize;
        bs = &bs[ret..];
    }
}
