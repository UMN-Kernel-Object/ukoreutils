#![no_std]
#![allow(internal_features)]
#![feature(lang_items, start)]

extern crate alloc;

pub mod io;
mod libc_alloc;
mod macros;
mod panic;
pub mod prelude;
mod start;

use core::{
    ffi::{c_int, CStr},
    fmt::Write,
};

/// Prints the message associated with the given errno and exits.
pub fn die_with_errno(errno: c_int, what: &str) -> ! {
    unsafe {
        let err_msg = CStr::from_ptr(libc::strerror(errno));
        eprintln!("failed to {}: {:?}", what, err_msg);
        libc::exit(111);
    }
}
