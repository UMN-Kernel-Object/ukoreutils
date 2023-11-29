#![no_std]

use core::ffi::CStr;
use ukoreutils::{io::stderr, prelude::*};

fn main() {
    let args = parse_args();
    todo!("{:#?}", args)
}

#[derive(Debug, Default)]
struct Args {
    path: &'static CStr,
    pred: Option<Predicate>,
}

#[derive(Debug)]
enum Predicate {
    StartsWith(&'static CStr),
    EndsWith(&'static CStr),
    IsDir,
    IsFile,
}

fn parse_args() -> Args {
    let mut out = Args::default();

    // Get the program arguments, skipping argv[0] (the name of the program).
    let mut args = args();
    let argv0 = args[0];
    args = &args[1..];

    // Store the path to search, if it exists.
    if let Some(path) = args.get(0) {
        out.path = path;
        args = &args[1..];
    } else {
        out.path = CStr::from_bytes_with_nul(b".\0").unwrap();
        return out;
    }

    // Check if there's a predicate.
    if !args.is_empty() {
        out.pred = match args[0].to_bytes() {
            b"--starts-with" if args.len() == 2 => Some(Predicate::StartsWith(args[1])),
            b"--ends-with" if args.len() == 2 => Some(Predicate::EndsWith(args[1])),
            b"--dir" if args.len() == 1 => Some(Predicate::IsDir),
            b"--file" if args.len() == 1 => Some(Predicate::IsFile),
            _ => usage_and_die(argv0),
        };
    }

    out
}

fn usage_and_die(argv0: &CStr) -> ! {
    eprint!("USAGE\n\n  ");
    stderr().write_bytes(argv0.to_bytes()).unwrap();
    eprintln!(concat!(
        " [PATH [PREDICATE]]\n\n",
        "PREDICATE\n",
        "\n  --starts-with STR  Find files whose names start with STR",
        "\n  --ends-with STR    Find files whose names end with STR",
        "\n  --dir              Find directories",
        "\n  --file             Find regular files",
    ));
    unsafe { libc::exit(101) }
}
