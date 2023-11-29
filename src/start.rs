use alloc::vec::Vec;
use core::{
    ffi::CStr,
    ptr::null_mut,
    slice,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};
use libc::c_int;

static ARGC: AtomicUsize = AtomicUsize::new(0);
static ARGV: AtomicPtr<&'static CStr> = AtomicPtr::new(null_mut());

#[lang = "start"]
pub fn start<T: Termination>(
    main: fn() -> T,
    argc: isize,
    argv: *const *const u8,
    // https://github.com/rust-lang/rust/issues/97889
    _sigpipe: u8,
) -> isize {
    // Collect the arguments as CStrs into a vector.
    let args = (0..argc)
        .map(|i| unsafe { CStr::from_ptr(*argv.offset(i) as *const i8) })
        .collect::<Vec<_>>();

    // Leak the vector to get back a slice of static lifetime.
    let args = args.leak();

    // Store the pointer and length of the slice in globals.
    ARGV.store(args.as_mut_ptr(), Ordering::SeqCst);
    ARGC.store(args.len(), Ordering::SeqCst);

    main().report() as isize
}

/// Returns the arguments the program was called with.
pub fn args() -> &'static [&'static CStr] {
    let argc = ARGC.load(Ordering::SeqCst);
    let argv = ARGV.load(Ordering::SeqCst);

    // SAFETY: start is the only writer to these variables, and it must have written valid values
    // to them before any other code runs.
    unsafe { slice::from_raw_parts(argv, argc as usize) }
}

pub trait Termination {
    fn report(self) -> c_int;
}

impl Termination for () {
    fn report(self) -> c_int {
        0
    }
}
