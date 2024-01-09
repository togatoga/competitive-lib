use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod segment_tree {

    /// `SegmentTreePURQ` stands for Segment Tree Point Update Range Query.
    /// interval addition, interval assignment, interval chmin, and interval chmax.
    pub struct SegmentTreePURQ<T, F> {
        seg: Vec<T>,
        size: usize,
        op: F,
        e: T,
    }
    impl<T: Copy, F: Fn(T, T) -> T> SegmentTreePURQ<T, F> {
        /// Makes a new `SegmentTreePURQ`.
        /// Example
        /// ```
        /// let add = SegmentTreePURQ::new(10, 0, |data, x| data + x);
        /// let max = SegmentTreePURQ::new(10, std::i64::MIN, |data, x| std::cmp::max(data, x));
        /// let min = SegmentTreePURQ::new(10, std::i64::MAX, |data, x| std::cmp::min(data, x));
        /// ```
        pub fn new(n: usize, e: T, op: F) -> SegmentTreePURQ<T, F> {
            let mut m = 1;
            while m <= n {
                m <<= 1;
            }
            SegmentTreePURQ {
                seg: vec![e; 2 * m],
                size: m,
                op,
                e,
            }
        }
        /// Updates the value of the index `k` to `value`.
        pub fn update(&mut self, k: usize, value: T) {
            let mut k = k;
            k += self.size - 1;
            self.seg[k] = value;
            while k > 0 {
                k = (k - 1) >> 1;
                self.seg[k] = (self.op)(self.seg[2 * k + 1], self.seg[2 * k + 2]);
            }
        }
        /// Gets the value of the interval [left, right).
        pub fn get(&self, left: usize, right: usize) -> T {
            assert!(left < right);
            self.get_range(left, right, 0, 0, self.size)
        }
        fn get_range(
            &self,
            left: usize,
            right: usize,
            k: usize,
            left_bound: usize,
            right_bound: usize,
        ) -> T {
            if right_bound <= left || right <= left_bound {
                self.e
            } else if left <= left_bound && right_bound <= right {
                self.seg[k]
            } else {
                let x = self.get_range(
                    left,
                    right,
                    2 * k + 1,
                    left_bound,
                    (left_bound + right_bound) >> 1,
                );
                let y = self.get_range(
                    left,
                    right,
                    2 * k + 2,
                    (left_bound + right_bound) >> 1,
                    right_bound,
                );
                (self.op)(x, y)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::segment_tree::SegmentTreePURQ;
    use rand::{thread_rng, Rng};

    #[test]
    fn random_update_and_max() {
        const N: usize = 1000;
        //max segment tree
        let mut seg = SegmentTreePURQ::new(N, 0, std::cmp::max);
        let mut values = vec![0; N];
        for _ in 0..10000 {
            let value = thread_rng().gen_range(0, 1000);
            let k = thread_rng().gen_range(0, N);
            values[k] = value;
            seg.update(k, value);
            //[0, n)
            assert_eq!(seg.get(0, N), *values.iter().max().unwrap());

            let l = thread_rng().gen_range(0, N);
            let r = thread_rng().gen_range(l, N);
            //two point
            //[l, r + 1)
            let x = values.iter().skip(l).take(r - l + 1).max().unwrap();
            assert_eq!(seg.get(l, r + 1), *x);
        }
    }

    #[test]
    fn random_update_and_sum() {
        const N: usize = 1000;
        // max segment tree
        let mut seg = SegmentTreePURQ::new(N, 0, |x, y| x + y);
        let mut values = vec![0; N];
        for _ in 0..10000 {
            let value = thread_rng().gen_range(0, 1000);
            let k = thread_rng().gen_range(0, N);
            values[k] = value;
            seg.update(k, value);
            // [0, n)
            assert_eq!(seg.get(0, N), values.iter().sum::<i32>());

            let l = thread_rng().gen_range(0, N);
            let r = thread_rng().gen_range(l, N);
            // two point
            // [l, r + 1)
            let x = values.iter().skip(l).take(r - l + 1).sum::<i32>();
            assert_eq!(seg.get(l, r + 1), x);
        }
    }
}
