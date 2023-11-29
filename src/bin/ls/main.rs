#![no_std]

use core::ffi::CStr;
use ukoreutils::{io::stderr, prelude::*};

fn main() {
    let args = parse_args();
    todo!("{:#?}", args)
}

#[derive(Debug)]
struct Args {
    sep: u8,
    all: bool,
    path: &'static CStr,
}

fn parse_args() -> Args {
    let mut out = Args {
        sep: b'\n',
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
                b'0' => out.sep = b'\0',
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

    out.path = match args.len() {
        0 => CStr::from_bytes_with_nul(b".\0").unwrap(),
        1 => args[0],
        _ => usage_and_die(argv0),
    };

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
