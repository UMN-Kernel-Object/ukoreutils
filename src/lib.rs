#![no_std]
#![allow(internal_features)]
#![feature(lang_items, start)]

pub mod io;
mod macros;
mod panic;
pub mod prelude;
mod start;

pub fn zero() -> isize {
    42
}

pub fn exit_zero() {
    unsafe { libc::exit(zero() as i32) }
}
