use cargo_snippet::snippet;
#[snippet]
#[allow(clippy::module_inception)]
pub mod range_set {
    use std::collections::BTreeSet;

    #[derive(Debug, Clone, Default)]
    pub struct RangeSet {
        set: BTreeSet<(i64, i64)>,
    }

    impl RangeSet {
        /// Returns a boolean whether the range [l, r] is covered by `set`.
        pub fn range_covered(&self, l: i64, r: i64) -> bool {
            assert!(l <= r);
            if let Some(&(left, right)) = self.set.range(..(l + 1, l + 1)).next_back() {
                left <= l && r <= right
            } else {
                false
            }
        }
        /// Returns a boolean whether a point is covered by `set`.
        pub fn covered(&self, x: i64) -> bool {
            self.range_covered(x, x)
        }

        /// Returns a range if it covers [l, r], otherwise returns `None`.
        pub fn range_covered_by(&self, l: i64, r: i64) -> Option<(i64, i64)> {
            assert!(l <= r);
            if let Some(&(left, right)) = self.set.range(..(l + 1, l + 1)).next_back() {
                (left <= l && r <= right).then_some((left, right))
            } else {
                None
            }
        }
        /// Returns a range if it covers a point, otherwise returns `None`.
        pub fn covered_by(&self, x: i64) -> Option<(i64, i64)> {
            self.range_covered_by(x, x)
        }

        /// Insert a range [l, r] to `set` and returns an increased amount.
        pub fn insert_range(&mut self, l: i64, mut r: i64) -> i64 {
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

        /// Insert a point into `set`.
        pub fn insert(&mut self, x: i64) -> i64 {
            self.insert_range(x, x)
        }
        /// Returns the number of range.
        pub fn size(&self) -> usize {
            self.set.len()
        }
        /// Returns an iter
        pub fn iter(&self) -> std::collections::btree_set::Iter<(i64, i64)> {
            self.set.iter()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_set::RangeSet;
    #[test]
    fn test_insert() {
        let mut set = RangeSet::default();
        let increased = set.insert_range(0, 5);
        assert!(increased == 6);
        let increased = set.insert_range(6, 7);
        assert!(increased == 2);
        let increased = set.insert(9);
        assert!(increased == 1);
        // [0, 7] [9, 9]
        assert!(set.size() == 2);

        // [0, 10]
        set.insert_range(8, 10);
        assert!(set.size() == 1);
    }
    #[test]
    fn test_random_insert() {}
}
