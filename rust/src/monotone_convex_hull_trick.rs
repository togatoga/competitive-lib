use cargo_snippet::snippet;
#[snippet]
#[allow(clippy::module_inception)]
pub mod monotone_convex_hull_trick {
    use std::collections::VecDeque;
    #[derive(Debug, Default, Clone)]
    pub struct MonotoneConvexHullTrick {
        lines: VecDeque<(i64, i64)>,
    }
    fn sgn(x: i64) -> i64 {
        if x < 0 {
            -1
        } else if x > 0 {
            1
        } else {
            0
        }
    }
    /// Returns a boolean whether a line `b` is unused.
    fn check(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        let (a1, b1) = a;
        let (a2, b2) = b;
        let (a3, b3) = c;
        if b2 == b1 || b2 == b3 {
            sgn(a2 - a1) * sgn(b3 - b2) >= sgn(a3 - a2) * sgn(b2 - b1)
        } else {
            (a2 - a1) as i128 * (b3 - b2) as i128 >= (a3 - a2) as i128 * (b2 - b1) as i128
        }
    }
    /// Returns a value a*x + b
    fn get_y(ab: (i64, i64), x: i64) -> i64 {
        let (a, b) = ab;
        a * x + b
    }
    impl MonotoneConvexHullTrick {
        /// Returns a new `MonotoneConvexHullTrick` from an array `ab` that are line segments.
        /// (a, b) a*x + b.
        /// An array `ab` must keep this condition.        
        /// `a` must be broadly monotonically increasing or broadly monotonically decreasing.
        pub fn new(ab: &[(i64, i64)]) -> MonotoneConvexHullTrick {
            let mut cht = MonotoneConvexHullTrick::default();
            for &ab in ab.iter() {
                cht.add(ab);
            }
            cht
        }

        /// Adds a line `a*x+ b`.
        /// `a` must be broadly monotonically increasing or broadly monotonically decreasing.
        pub fn add(&mut self, ab: (i64, i64)) {
            let (a, b) = ab;
            if self.lines.is_empty() {
                self.lines.push_front((a, b));
                return;
            }
            if self.lines[0].0 <= a {
                let (a1, b1) = self.lines[0];
                if a1 == a {
                    if b1 <= b {
                        return;
                    }
                    self.lines.pop_front();
                }
                while self.lines.len() >= 2 && check((a, b), self.lines[0], self.lines[1]) {
                    self.lines.pop_front();
                }
                self.lines.push_front((a, b));
            } else {
                let (a2, b2) = self.lines[self.lines.len() - 1];
                if a2 == a {
                    if b2 <= b2 {
                        return;
                    }
                    self.lines.pop_back();
                }
                while self.lines.len() >= 2
                    && check(
                        self.lines[self.lines.len() - 2],
                        self.lines[self.lines.len() - 1],
                        (a, b),
                    )
                {
                    self.lines.pop_back();
                }
                self.lines.push_back((a, b));
            }
        }
        /// Returns a minimum value of `a*x + b` for given line segments.
        pub fn query(&self, x: i64) -> i64 {
            assert!(!self.lines.is_empty());
            let mut left = -1_isize;
            let mut right = (self.lines.len() - 1) as isize;
            while right - left > 1 {
                let med = ((left + right) >> 1) as usize;
                let v1 = get_y(self.lines[med], x);
                let v2 = get_y(self.lines[med + 1], x);
                if v1 >= v2 {
                    left = med as isize;
                } else {
                    right = med as isize;
                }
            }
            get_y(self.lines[right as usize], x)
        }
        /// Clears the line segment set.
        pub fn clear(&mut self) {
            self.lines.clear()
        }
        /// Returns a boolean whether a line segment set is empty.
        pub fn is_empty(&self) -> bool {
            self.lines.is_empty()
        }
        /// Returns a minimu value of `a*x + b` for given line segments.
        /// An argument of `x` must be broadly monotonically increasing.
        pub fn query_monotone_inc(&mut self, x: i64) -> i64 {
            assert!(!self.lines.is_empty());
            while self.lines.len() >= 2 && get_y(self.lines[0], x) >= get_y(self.lines[1], x) {
                self.lines.pop_front();
            }
            get_y(self.lines[0], x)
        }
        /// Returns a minimu value of `a*x + b` for given line segments.
        /// An argument of `x` must be broadly monotonically decreasing.
        pub fn query_monotone_dec(&mut self, x: i64) -> i64 {
            assert!(!self.lines.is_empty());
            while self.lines.len() >= 2
                && get_y(self.lines[self.lines.len() - 1], x)
                    >= get_y(self.lines[self.lines.len() - 2], x)
            {
                self.lines.pop_back();
            }
            get_y(self.lines[self.lines.len() - 1], x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::monotone_convex_hull_trick::MonotoneConvexHullTrick;

    #[test]
    fn test() {
        let mut cht = MonotoneConvexHullTrick::default();
        cht.add((2, 0));
        // y = 2*x
        assert_eq!(cht.query(-2), -4);
        assert_eq!(cht.query(2), 4);
        // y = 2*x, y = 3*x + 4
        cht.add((3, 4));
        assert_eq!(cht.query(2), 4);
        assert_eq!(cht.query(-5), -11);
        // y = 2*x, y = 3*x + 4, y = 7*x - 10
        cht.add((7, -10));
        assert_eq!(cht.query(0), -10);
        assert_eq!(cht.query(1), -3);
    }
}
