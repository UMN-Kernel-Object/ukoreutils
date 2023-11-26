#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        write!($crate::io::stderr(), $($arg)*).unwrap()
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        writeln!($crate::io::stderr(), $($arg)*).unwrap()
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        write!($crate::io::stdout(), $($arg)*).unwrap()
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        writeln!($crate::io::stdout(), $($arg)*).unwrap()
    };
}
