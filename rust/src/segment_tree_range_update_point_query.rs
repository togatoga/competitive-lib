use cargo_snippet::snippet;
#[snippet]
#[allow(clippy::module_inception)]
/// An original implementation is `https://atcoder.jp/contests/abc332/submissions/48383924`
/// Verified by `https://atcoder.jp/contests/abc332/submissions/49187833`
pub mod segment_tree_rupq {
    /// `SegmentTreeRUPQ` stands for Segment Tree Range Update Point Query.
    pub struct SegmentTreeRUPQ<T, F> {
        size: usize,
        bit: usize,
        data: Vec<T>,
        e: T,
        op: F,
    }

    impl<T, F> SegmentTreeRUPQ<T, F>
    where
        T: Clone,
        F: Fn(&T, &T) -> T,
    {
        /// Makes a new `SegmentTreeRUPQ`.
        /// Example
        /// ```
        /// let add = SegmentTreeRUPQ::new(10, 0, |data, x| data + x);
        /// let max = SegmentTreeRUPQ::new(10, std::i64::MIN, |data, x| std::cmp::max(data, x));
        /// let min = SegmentTreeRUPQ::new(10, std::i64::MAX, |data, x| std::cmp::min(data, x));
        /// ```        
        pub fn new(size: usize, e: T, op: F) -> Self {
            let size = size.next_power_of_two();
            let bit = size.trailing_zeros() as usize;
            Self {
                size,
                bit,
                data: vec![e.clone(); 2 * size],
                e,
                op,
            }
        }

        /// Gets the value of the index `x`.
        pub fn get(&self, x: usize) -> T {
            assert!(x < self.size);
            let mut x = x + self.size;
            let mut ans = self.data[x].clone();
            while x > 1 {
                x >>= 1;
                ans = (self.op)(&ans, &self.data[x]);
            }
            ans
        }

        /// Updates the interval (l, r] with f.
        pub fn update(&mut self, l: usize, r: usize, f: T) {
            assert!(l <= r && r <= self.size);
            if l == r {
                return;
            }
            let mut l = l + self.size;
            let mut r = r + self.size;
            for i in (1..=self.bit).rev() {
                if (l >> i) << i != l {
                    self.propagate(l >> i);
                }
                if (r >> i) << i != r {
                    self.propagate((r - 1) >> i);
                }
            }
            while l < r {
                if l & 1 == 1 {
                    self.data[l] = (self.op)(&self.data[l], &f);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.data[r] = (self.op)(&self.data[r], &f);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        fn propagate(&mut self, x: usize) {
            let f = std::mem::replace(&mut self.data[x], self.e.clone());
            self.data[2 * x] = (self.op)(&self.data[2 * x], &f);
            self.data[2 * x + 1] = (self.op)(&self.data[2 * x + 1], &f);
        }
    }
}
