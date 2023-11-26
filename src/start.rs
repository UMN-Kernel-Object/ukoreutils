use libc::c_int;

#[lang = "start"]
pub fn start<T: Termination>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    // https://github.com/rust-lang/rust/issues/97889
    _sigpipe: u8,
) -> isize {
    main().report() as isize
}

pub trait Termination {
    fn report(self) -> c_int;
}

impl Termination for () {
    fn report(self) -> c_int {
        0
    }
}
