use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod change_min_max {
    pub trait ChangeMinMax<T: PartialOrd> {
        /// Change `self` to the minimum of two values if `self` is less than `value`(`self` < `value`).
        /// Returns a boolean whether `self` is changed.
        fn chmin(&mut self, value: T) -> bool;
        /// Change `self` to the maximum of two values if `self` is more than `value`(`self` > `value`).
        /// Returns a boolean whether `self` is changed.
        fn chmax(&mut self, value: T) -> bool;
    }

    impl<T: PartialOrd> ChangeMinMax<T> for T {
        fn chmin(&mut self, value: T) -> bool {
            if value.lt(&self) {
                *self = value;
                true
            } else {
                false
            }
        }
        fn chmax(&mut self, value: T) -> bool {
            if value.gt(&self) {
                *self = value;
                true
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::change_min_max::ChangeMinMax;
    #[test]
    pub fn test_change_min_max() {
        let mut x = 10;

        // change min
        assert!(x.chmin(5));
        assert_eq!(x, 5);

        // do nothing
        assert!(!x.chmin(5));
        assert_eq!(x, 5);

        // change max
        assert!(x.chmax(100));
        assert_eq!(x, 100);

        // do nothing
        assert!(!x.chmax(100));
        assert_eq!(x, 100);
    }
}
