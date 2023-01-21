use cargo_snippet::snippet;
#[snippet]
#[allow(clippy::module_inception)]
/// verified@https://codeforces.com/contest/915/submission/189948413
pub mod range_set {
    use std::{
        collections::BTreeSet,
        ops::{Range, RangeInclusive},
    };

    #[derive(Debug, Clone, Default)]
    pub struct RangeSet {
        set: BTreeSet<(i64, i64)>,
    }

    pub trait ClosedInterval {
        fn interval(&self) -> (i64, i64);
    }

    impl ClosedInterval for Range<i64> {
        fn interval(&self) -> (i64, i64) {
            (self.start, self.end - 1)
        }
    }
    impl ClosedInterval for RangeInclusive<i64> {
        fn interval(&self) -> (i64, i64) {
            (*self.start(), *self.end())
        }
    }
    impl ClosedInterval for i64 {
        fn interval(&self) -> (i64, i64) {
            (*self, *self)
        }
    }

    impl RangeSet {
        /// Returns a boolean whether a given range is fully covered by `set`.
        pub fn covered<T: ClosedInterval>(&self, range: T) -> bool {
            let (l, r) = range.interval();
            assert!(l <= r);
            if let Some(&(left, right)) = self.set.range(..(l + 1, l + 1)).next_back() {
                left <= l && r <= right
            } else {
                false
            }
        }

        /// Returns a range if it covers, otherwise returns `None`.
        pub fn covered_by<T: ClosedInterval>(&self, range: T) -> Option<(i64, i64)> {
            let (l, r) = range.interval();
            assert!(l <= r);
            if let Some(&(left, right)) = self.set.range(..(l + 1, l + 1)).next_back() {
                if left <= l && r <= right {
                    Some((left, right))
                } else {
                    None
                }
            } else {
                None
            }
        }

        /// Inserts a range into `set` and returns an inserted amount.
        pub fn insert<T: ClosedInterval>(&mut self, range: T) -> i64 {
            let (l, mut r) = range.interval();
            assert!(l <= r);
            // erase
            let mut erased = 0;
            while let Some(&(left, right)) = self.set.range((l + 1, l + 1)..).next() {
                if r + 1 == left || left <= r {
                    r = std::cmp::max(r, right);
                    self.set.remove(&(left, right));
                    erased += right - left + 1;
                } else {
                    break;
                }
            }

            // insert
            let mut inserted = 0;
            let r1 = self.set.range(..(l + 1, l + 1)).next_back();
            if let Some(&(left, right)) = r1 {
                // fully covered
                if left <= l && r <= right {
                    return 0;
                } else {
                    // merge
                    // [left, right] [l, r] => [left, r]
                    if right + 1 == l || l <= right {
                        self.set.remove(&(left, right));
                        erased += right - left + 1;

                        self.set.insert((left, r));
                        inserted += r - left + 1;
                    } else {
                        self.set.insert((l, r));
                        inserted += r - l + 1;
                    }
                }
            } else {
                self.set.insert((l, r));
                inserted += r - l + 1;
            }
            inserted - erased
        }

        /// Erases a range and returns an erased amount.
        pub fn erase<T: ClosedInterval>(&mut self, range: T) -> i64 {
            let (l, r) = range.interval();
            assert!(l <= r);

            // fully covered
            if let Some((left, right)) = self.covered_by(range) {
                self.set.remove(&(left, right));
                if left < l {
                    self.set.insert((left, l - 1));
                }
                if r < right {
                    self.set.insert((r + 1, right));
                }
                return r - l + 1;
            }
            // erase right ranges
            let mut erased = 0;
            let mut inserted = 0;
            while let Some(&(left, right)) = self.set.range((l + 1, l + 1)..).next() {
                // not intersected
                if r < left {
                    break;
                }
                erased += right - left + 1;
                self.set.remove(&(left, right));
                if r < right {
                    self.set.insert((r + 1, right));
                    inserted += right - r;
                    break;
                }
            }

            // erase a left range
            if let Some(&(left, right)) = self.set.range(..(l + 1, l + 1)).next_back() {
                if l <= right {
                    self.set.remove(&(left, right));
                    erased += right - left + 1;
                    if left != l {
                        self.set.insert((left, l - 1));
                        inserted += l - left;
                    }
                }
            }
            erased - inserted
        }

        /// Returns a mex that is greater than `x`.
        pub fn mex(&self, x: i64) -> i64 {
            if let Some(&(left, right)) = self.set.range(..(x + 1, x + 1)).next_back() {
                if left <= x && x <= right {
                    right + 1
                } else {
                    x
                }
            } else {
                x
            }
        }

        /// Returns the number of range.
        pub fn size(&self) -> usize {
            self.set.len()
        }
        /// Returns an iter
        pub fn iter(&self) -> std::collections::btree_set::Iter<(i64, i64)> {
            self.set.iter()
        }

        /// Clears `set`
        pub fn clear(&mut self) {
            self.set.clear();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::range_set::RangeSet;
    #[test]
    fn test_insert_erase() {
        let mut set = RangeSet::default();
        // [0,5]
        let increased = set.insert(0..=5);
        assert!(increased == 6);
        assert_eq!(set.mex(0), 6);
        // [0, 7]
        let increased = set.insert(6..=7);
        assert!(increased == 2);
        assert_eq!(set.mex(0), 8);

        // [0, 7] [9, 9]
        let increased = set.insert(9);
        assert!(increased == 1);
        assert!(set.size() == 2);
        assert_eq!(set.mex(0), 8);

        // [0, 10]
        set.insert(8..=10);
        assert!(set.size() == 1);
        assert_eq!(set.mex(0), 11);
        assert_eq!(set.mex(-100), -100);

        // erase
        // [0, 1] [6, 10]
        let erased = set.erase(2..=5);
        assert_eq!(erased, 4);
        assert_eq!(set.size(), 2);
        // [1, 1] [6, 10];
        let erased = set.erase(0);
        assert_eq!(erased, 1);
        assert_eq!(set.size(), 2);
        // [1, 1]
        let erased = set.erase(2..=10);
        assert_eq!(erased, 5);
        assert_eq!(set.size(), 1);

        // []
        let erased = set.erase(1);
        assert_eq!(erased, 1);
        assert_eq!(set.size(), 0);

        // [8, 8]
        assert_eq!(set.insert(8), 1);
        // [1, 2] [8, 8]
        assert_eq!(set.insert(1..=2), 2);
        // [8, 8]
        assert_eq!(set.erase(1..=7), 2);
        // [8, 8]
        assert_eq!(set.erase(0..=5), 0);

        // [8, 12]
        assert_eq!(set.insert(9..13), 4);
        assert_eq!(set.size(), 1);
        // [8, 9] [12]
        assert_eq!(set.erase(10..12), 2);
    }

    #[test]
    fn test_random_insert_erase() {
        use rand::{thread_rng, Rng};
        let n: usize = 5000;

        let mut covered = vec![false; n + 1];
        let mut set = RangeSet::default();
        for _ in 0..=n / 10 {
            let left = thread_rng().gen_range(0, n);
            let right = thread_rng().gen_range(left, n);

            let value = thread_rng().gen_range(0, 2);
            // println!("{} {} {}", left, right, value);
            if value % 2 == 0 {
                // insert
                let mut increased = 0;
                covered.iter_mut().take(right + 1).skip(left).for_each(|c| {
                    if !*c {
                        increased += 1;
                    }
                    *c = true;
                });

                assert_eq!(set.insert(left as i64..=right as i64), increased);
            } else {
                // insert
                let mut erased = 0;
                covered.iter_mut().take(right + 1).skip(left).for_each(|c| {
                    if *c {
                        erased += 1;
                    }
                    *c = false;
                });
                assert_eq!(set.erase(left as i64..=right as i64), erased);
            }

            // checked coverd ranges
            for (left, right) in set
                .iter()
                .map(|&(left, right)| (left as usize, right as usize))
            {
                assert!(covered.iter().skip(left).take(right - left + 1).all(|&x| x));
            }

            let mut checked_mex = false;
            for i in 0..n {
                assert_eq!(set.covered(i as i64..=i as i64), covered[i]);
                if covered[i] {
                    let (left, right) = set.covered_by(i as i64..=i as i64).expect("no range");
                    if left >= 1 {
                        assert!(!covered[(left - 1) as usize]);
                    }
                    if right as usize + 1 < n {
                        assert!(!covered[(right + 1) as usize]);
                    }
                } else {
                    if !checked_mex {
                        assert_eq!(set.mex(0), i as i64);
                    }
                    checked_mex = true;
                }
            }
        }
    }
}
