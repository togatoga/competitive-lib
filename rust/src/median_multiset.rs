use crate::btree_multiset::btree_multiset::BTreeMultiSet;
use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(
    include = "btree_multiset",
    prefix = "use crate::btree_multiset::BTreeMultiSet;"
)]
/// Calcualte a median value in the set.
/// verified: https://atcoder.jp/contests/abc218/submissions/25807030
pub mod median_multiset {

    use super::*;
    use std::iter::FromIterator;

    /// `MedianMultiSet` calcualtes a median value in the set.
    #[derive(Debug, Default, Clone)]
    pub struct MedianMultiSet<T: Ord + Clone> {
        lower: BTreeMultiSet<T>,
        upper: BTreeMultiSet<T>,
    }

    impl<T: Ord + Clone> MedianMultiSet<T> {
        /// Makes, a new, empty `MedianMultiSet`
        pub fn new() -> MedianMultiSet<T> {
            MedianMultiSet {
                lower: BTreeMultiSet::new(),
                upper: BTreeMultiSet::new(),
            }
        }
        /// Adds a value to the set.
        pub fn insert(&mut self, value: T) {
            self.lower.insert(value);
            self.rebalance();
        }

        /// Removes a value from the set. Returns whether the value was present in the set
        pub fn remove(&mut self, value: &T) -> bool {
            let mut removed = self.upper.remove(value);
            if !removed {
                removed |= self.lower.remove(value);
            }
            self.rebalance();
            removed
        }

        /// Returns `true` if the set contains a value
        pub fn contains(&mut self, value: &T) -> bool {
            self.lower.contains(value) || self.upper.contains(value)
        }

        /// Clears the set
        pub fn clear(&mut self) {
            self.lower.clear();
            self.upper.clear();
        }

        /// Returns a median value in the set.
        /// A returned value is a pair optional value.
        ///
        /// If the number of the set is
        /// odd : a first value is set. a second value is `None`
        /// even: a first and second value is set.
        pub fn median(&self) -> (Option<&T>, Option<&T>) {
            if self.lower.is_empty() && self.upper.is_empty() {
                return (None, None);
            }

            if self.lower.len() == self.upper.len() + 1 {
                (self.lower.last(), None)
            } else {
                (self.lower.last(), self.upper.first())
            }
        }

        /// Returns `true` if the set is empty.
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Returns the number of elements in the set.
        pub fn len(&self) -> usize {
            self.lower.len() + self.upper.len()
        }

        /// Rebalance `lower` and `upper` set.
        /// Make sure that the difference number between `lower` and `upper` must be less thatn equal to 1.
        fn rebalance(&mut self) {
            if let Some(last) = self.lower.pop_last() {
                self.upper.insert(last);
            }
            while self.lower.len() < self.upper.len() {
                if let Some(v) = self.upper.pop_first() {
                    self.lower.insert(v);
                }
            }
            assert!(self.lower.len() - self.upper.len() <= 1);
        }
    }

    impl<T: Ord + Clone> FromIterator<T> for MedianMultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut mset = MedianMultiSet::new();
            for x in iter {
                mset.insert(x);
            }
            mset
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::median_multiset::MedianMultiSet;

    #[test]
    fn test_insert_remove() {
        let mut set = MedianMultiSet::new();
        assert_eq!(set.median(), (None, None));
        set.insert(0);
        set.insert(1);
        set.insert(2);
        // [0, 1, 2]
        assert_eq!(set.median(), (Some(&1), None));
        set.remove(&2);
        // [0, 1]
        assert_eq!(set.median(), (Some(&0), Some(&1)));

        set.insert(1);
        set.insert(2);
        set.insert(2);
        // [0, 1, 1, 2, 2]
        assert_eq!(set.median(), (Some(&1), None));
        assert_eq!(set.len(), 5);

        let elements = vec![1, 2, 2, 3, 4, 5, 5, 6];
        let set = MedianMultiSet::from_iter(elements);
        assert_eq!(set.median(), (Some(&3), Some(&4)));
    }
}
