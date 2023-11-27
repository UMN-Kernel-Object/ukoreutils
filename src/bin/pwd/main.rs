#![no_std]

use core::ffi::{c_char, CStr};
use ukoreutils::{io::stdout, prelude::*};

fn main() {
    let mut buf = [0u8; libc::PATH_MAX as usize];
    unsafe {
        let out = libc::getcwd(buf.as_mut_ptr() as *mut c_char, buf.len());
        if out.is_null() {
            let err = ukoreutils::io::errno();
            let err_msg = CStr::from_ptr(libc::strerror(err));
            eprintln!("failed to get cwd: {:?}", err_msg);
            libc::exit(111);
        }
    }

    let cwd = CStr::from_bytes_until_nul(&buf[..]).expect("getcwd() behaved incorrectly");
    stdout()
        .write_bytes(cwd.to_bytes())
        .expect("failed to write cwd");
    println!();
}
