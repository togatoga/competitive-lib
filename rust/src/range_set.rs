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
        /// Returns a range if it covers [l, r], otherwise returns `None`
        pub fn covered_by(&self, l: i64, r: i64) -> Option<(i64, i64)> {
            assert!(l <= r);
            if let Some(&(left, right)) = self.set.range(..(l + 1, l + 1)).next_back() {
                (left <= l && r <= right).then_some((left, right))
            } else {
                None
            }
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
        pub fn range_size(&self) -> usize {
            self.set.len()
        }
    }
}
