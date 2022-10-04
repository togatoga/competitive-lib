use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod macros {

    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! max {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            std::cmp::max($x, max!($($y),+))
        }
    }

    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! min {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            std::cmp::min($x, min!($($y),+))
        }
    }
    #[macro_export]
    #[allow(unused_macros)]
    /// Display a line of variables
    macro_rules! echo {
        () => {
            if cfg!(debug_assertions) {
                use std::io::{self, Write};
                writeln!(io::stderr(), "{}:", line!()).unwrap();
            }
        };
        ($e: expr, $($es: expr),+ $(,)?) => {
            if cfg!(debug_assertions) {
                use std::io::{self, Write};

                write!(io::stderr(), "{}:", line!()).unwrap();
                write!(io::stderr(), " {} = {:?}", stringify!($e), $e).unwrap();
                $(
                    write!(io::stderr(), " {} = {:?}", stringify!($es), $es).unwrap();
                )+
                writeln!(io::stderr()).unwrap();
            }
        };

        ($e: expr) => {
            if cfg!(debug_assertions) {
                use std::io::{self, Write};
                let result = $e;
                writeln!(io::stderr(), "{}: {} = {:?}",
                        line!(), stringify!($e), result)
                    .unwrap();
            }
        };
    }

    #[macro_export]
    #[allow(unused_macros)]
    /// Display a line of variables with colors
    macro_rules! cecho {
        () => {
            if cfg!(debug_assertions) {
                use std::io::{self, Write};
                writeln!(io::stderr(), "\x1b[31;1m{}\x1b[m:", line!()).unwrap();
            }
        };
        ($e: expr, $($es: expr),+ $(,)?) => {
            if cfg!(debug_assertions) {
                use std::io::{self, Write};

                write!(io::stderr(), "\x1b[31;1m{}\x1b[m:", line!()).unwrap();
                write!(io::stderr(), " \x1b[92;1m{}\x1b[m = {:?}", stringify!($e), $e).unwrap();
                $(
                    write!(io::stderr(), " \x1b[92;1m{}\x1b[m = {:?}", stringify!($es), $es).unwrap();
                )+
                writeln!(io::stderr()).unwrap();
            }
        };

        ($e: expr) => {
            if cfg!(debug_assertions) {
                use std::io::{self, Write};
                let result = $e;
                writeln!(io::stderr(), "\x1b[31;1m{}\x1b[m: \x1b[92;1m{}\x1b[m = {:?}",
                        line!(), stringify!($e), result)
                    .unwrap();
            }
        };
    }
}
