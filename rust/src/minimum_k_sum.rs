use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod minimum_k_sum {
    use std::cmp::Reverse;
    use std::fmt::Debug;

    /// `MinimumKSum` is a priority queue that calculates the sum of the k smallest elements.
    #[derive(Debug, Clone)]
    pub struct MinimumKSum<T> {
        values: std::collections::BinaryHeap<T>,
        deleted_values: std::collections::BinaryHeap<T>,
        cands: std::collections::BinaryHeap<Reverse<T>>,
        deleted_cands: std::collections::BinaryHeap<Reverse<T>>,
        /// 1-indexed
        k: usize,
        sum: T,
    }

    impl<T> MinimumKSum<T>
    where
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::cmp::Ord
            + std::ops::Neg<Output = T>
            + Copy
            + Debug
            + std::cmp::PartialOrd
            + std::ops::AddAssign
            + std::ops::SubAssign,
    {
        /// return sum of k largest elements
        pub fn new(k: usize, zero: T) -> Self {
            MinimumKSum {
                values: std::collections::BinaryHeap::new(),
                deleted_values: std::collections::BinaryHeap::new(),
                cands: std::collections::BinaryHeap::new(),
                deleted_cands: std::collections::BinaryHeap::new(),
                k,
                sum: zero,
            }
        }

        /// modify ins and d_ins to satisfy the condition
        fn modify(&mut self) {
            while (self.values.len() as isize - self.deleted_values.len() as isize)
                < self.k as isize
                && !self.cands.is_empty()
            {
                let x = self.cands.pop().expect("no element in outs");
                let y = self.deleted_cands.peek();
                if y.map_or(false, |&y| y.0 == x.0) {
                    self.deleted_cands.pop();
                } else {
                    self.sum += x.0;
                    self.values.push(x.0);
                }
            }
            while (self.values.len() as isize - self.deleted_values.len() as isize)
                > self.k as isize
            {
                let x = self.values.pop().expect("no element in ins");
                let y = self.deleted_values.peek();
                if y.map_or(false, |&y| y == x) {
                    self.deleted_values.pop();
                } else {
                    self.sum -= x;
                    self.cands.push(Reverse(x));
                }
            }

            loop {
                match (self.values.peek(), self.deleted_values.peek()) {
                    (Some(&x), Some(&y)) if x == y => {
                        self.values.pop();
                        self.deleted_values.pop();
                    }
                    _ => break,
                }
            }
        }

        /// return sum of k smallest elements
        pub fn sum(&self) -> T {
            self.sum
        }

        /// push x to the heap
        pub fn push(&mut self, x: T) {
            self.values.push(x);
            self.sum += x;
            self.modify();
        }

        /// erase x from the heap
        /// `x` must be in the heap
        pub fn erase(&mut self, x: T) {
            assert!(!self.is_empty());
            let y = self.values.peek();
            if y.map_or(false, |&y| y == x) {
                self.sum -= x;
                self.values.pop();
            } else if y.map_or(false, |&y| y > x) {
                self.sum -= x;
                self.deleted_values.push(x);
            } else {
                self.deleted_cands.push(Reverse(x));
            }
            self.modify();
        }

        /// set k
        pub fn set_k(&mut self, k: usize) {
            self.k = k;
            self.modify();
        }

        /// return k
        pub fn k(&self) -> usize {
            self.k
        }

        /// return the number of elements in the heap
        pub fn size(&self) -> usize {
            self.values.len() + self.cands.len()
                - self.deleted_values.len()
                - self.deleted_cands.len()
        }

        /// return true if the heap is empty
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
    }
}
