use cargo_snippet::snippet;
#[snippet]
/// These traits provides almost same methods that mordern Rust compiler provides and fill the gap between them.
pub mod gap_traits {
    /// We don't need this trait if we can use mordern rust compiler greater than equal to 1.50.
    pub trait BoolToOption {
        fn then_some<T>(self, t: T) -> Option<T>;
        fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
    }

    impl BoolToOption for bool {
        /// Returns `Some(t)` if the `bool` is `true`, or `None` otherwise.
        ///
        /// # Examples
        ///
        /// ```
        ///
        /// assert_eq!(false.then_some(0), None);
        /// assert_eq!(true.then_some(0), Some(0));
        /// ```
        fn then_some<T>(self, t: T) -> Option<T> {
            if self {
                Some(t)
            } else {
                None
            }
        }
        /// Returns `Some(f())` if the `bool` is `true`, or `None` otherwise.
        ///
        /// # Examples
        ///
        /// ```
        /// assert_eq!(false.then(|| 0), None);
        /// assert_eq!(true.then(|| 0), Some(0));
        /// ```
        fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
            if self {
                Some(f())
            } else {
                None
            }
        }
    }
}
