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
    macro_rules! ep {
        () => {
            {
                use std::io::Write;
                writeln!(std::io::stderr(), "\x1b[34;1m{}\x1b[m:", line!()).unwrap();
            }
        };
        ($e:expr, $($es:expr),+) => {
            {
                use std::io::Write;
                write!(std::io::stderr(), "\x1b[34;1m{}\x1b[m:", line!()).unwrap();
                write!(
                    std::io::stderr(),
                    " \x1b[92;1m{}\x1b[m = {:?}",
                    stringify!($e),
                    $e
                )
                .unwrap();
                $(
                    write!(std::io::stderr(), ", \x1b[92;1m{}\x1b[m = {:?}", stringify!($es), $es).unwrap();
                )+
                writeln!(std::io::stderr()).unwrap();
            }
        };

        ($e: expr) => {
            {
                use std::io::Write;
                let result = $e;
                writeln!(
                    std::io::stderr(),
                    "\x1b[34;1m{}\x1b[m: \x1b[92;1m{}\x1b[m = {:?}",
                    line!(),
                    stringify!($e),
                    result
                )
                .unwrap();
            }
        };
    }

    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! dep {
        () => {
            if cfg!(debug_assertions) {
                {
                    use std::io::Write;
                    write!(std::io::stderr(), "\x1b[31;1m{}\x1b[m ", "[DEBUG]").unwrap();
                }                
                ep!();
            }
        };
        ($e:expr, $($es:expr),+) => {
            if cfg!(debug_assertions) {
                {
                    use std::io::Write;
                    write!(std::io::stderr(), "\x1b[31;1m{}\x1b[m ", "[DEBUG]").unwrap();
                }
                ep!($e, $($es),+);
            }
        };

        ($e: expr) => {
            if cfg!(debug_assertions) {
                {
                    use std::io::Write;
                    write!(std::io::stderr(), "\x1b[31;1m{}\x1b[m ", "[DEBUG]").unwrap();
                }
                ep!($e);
            }
        };
    }
}
