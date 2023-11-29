#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, ffi::CString};
use core::ffi::{c_int, CStr};
use ukoreutils::{io::stderr, prelude::*};

fn main() {
    let args = parse_args();
    let r = (|| -> Result<(), c_int> {
        for path in args.directories {
            mkdir(path, args.parents)?;
        }
        Ok(())
    })();

    if let Err(err) = r {
        die_with_errno(err, "mkdir");
    }
}

fn mkdir(path: &CStr, parents: bool) -> Result<(), c_int> {
    loop {
        let ret = unsafe { libc::mkdir(path.as_ptr(), 0o777) };
        if ret == 0 {
            break Ok(());
        }
        let err = ukoreutils::io::errno();

        if parents && err == libc::ENOENT {
            let parent = dirname(path.to_owned());
            mkdir(&parent, parents)?;
            continue;
        } else if parents && err == libc::EEXIST {
            break Ok(());
        } else {
            break Err(err);
        }
    }
}

fn dirname(path: CString) -> CString {
    unsafe { CString::from_raw(libc::dirname(path.into_raw())) }
}

fn die_with_errno(err: c_int, what: &str) -> ! {
    unsafe {
        let err_msg = CStr::from_ptr(libc::strerror(err));
        eprintln!("failed to {}: {:?}", what, err_msg);
        libc::exit(111);
    }
}

#[derive(Debug, Default)]
struct Args {
    parents: bool,
    directories: &'static [&'static CStr],
}

fn parse_args() -> Args {
    let mut out = Args::default();

    // Get the program arguments, skipping argv[0] (the name of the program).
    let mut args = args();
    let argv0 = args[0];
    args = &args[1..];

    // Process options as long as they keep being at the front of the arguments.
    let mut options_done = false;
    while let Some(arg) = args.get(0) {
        let arg = arg.to_bytes();
        if !arg.starts_with(b"-") || options_done {
            break;
        }

        // Handle each option character.
        for ch in arg.iter().skip(1).copied() {
            match ch {
                b'-' => {
                    options_done = true;
                }
                b'p' => {
                    out.parents = true;
                }
                _ => {
                    eprintln!("unknown option: -{}", char::from(ch));
                    usage_and_die(argv0)
                }
            }
        }

        // Advance through the arguments.
        args = &args[1..];
    }

    if args.is_empty() {
        usage_and_die(argv0)
    }
    out.directories = args;

    out
}

fn usage_and_die(argv0: &CStr) -> ! {
    eprint!("USAGE\n\n  ");
    stderr().write_bytes(argv0.to_bytes()).unwrap();
    eprintln!(" [FLAGS] <PATHS...>\n\nFLAGS\n\n  -p\tCreate parent directories. No error if the directory exists");
    unsafe { libc::exit(101) }
}
