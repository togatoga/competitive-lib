use cargo_snippet::snippet;
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
}
