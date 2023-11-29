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

#[deprecated]
pub fn zero() -> isize {
    42
}

#[deprecated]
pub fn exit_zero() {
    unsafe { libc::exit(zero() as i32) }
}
