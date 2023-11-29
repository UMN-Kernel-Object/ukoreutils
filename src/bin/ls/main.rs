#![no_std]

use core::ffi::{c_int, CStr};
use ukoreutils::{
    io::{errno, stderr},
    prelude::*,
};

fn main() {
    let args = parse_args();
    let dir = unsafe { libc::opendir(args.path.as_ptr()) };
    if dir.is_null() {
        die_with_errno(errno(), "opendir")
    }

    todo!("{:#?}", (args, dir))
}

fn die_with_errno(err: c_int, what: &str) -> ! {
    unsafe {
        let err_msg = CStr::from_ptr(libc::strerror(err));
        eprintln!("failed to {}: {:?}", what, err_msg);
        libc::exit(111);
    }
}

#[derive(Debug)]
struct Args {
    sep: char,
    all: bool,
    path: &'static CStr,
}

fn parse_args() -> Args {
    let mut out = Args {
        sep: '\n',
        all: false,
        path: Default::default(),
    };

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
                b'-' => options_done = true,
                b'0' => out.sep = '\0',
                b'a' => out.all = true,
                _ => {
                    eprintln!("unknown option: -{}", char::from(ch));
                    usage_and_die(argv0)
                }
            }
        }

        // Advance through the arguments.
        args = &args[1..];
    }

    if args.len() != 1 {
        usage_and_die(argv0)
    }
    out.path = args[0];

    out
}

fn usage_and_die(argv0: &CStr) -> ! {
    eprint!("USAGE\n\n  ");
    stderr().write_bytes(argv0.to_bytes()).unwrap();
    eprintln!(concat!(
        " [FLAGS] <DIR-PATH>\n\n",
        "FLAGS\n",
        "\n  -0  Separate directories with null bytes, not newlines",
        "\n  -a  List hidden files"
    ));
    unsafe { libc::exit(101) }
}
