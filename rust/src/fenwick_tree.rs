use cargo_snippet::snippet;
#[snippet]
pub mod fenwick_tree {
    //Binary Indexed Tree
    use std::ops::*;
    pub struct FenwickTree<T> {
        values: Vec<T>,
        init_value: T,
    }

    impl<T: Copy + Clone + AddAssign + SubAssign + Sub<Output = T> + Ord> FenwickTree<T> {
        pub fn new(n: usize, init_value: T) -> FenwickTree<T> {
            FenwickTree {
                values: vec![init_value; n + 1],
                init_value,
            }
        }
        /// Caluculate the range sum [l, r)
        pub fn query(&self, l: usize, r: usize) -> T {
            self.sum(r) - self.sum(l)
        }
        /// Caluculate the range sum [0, i)
        pub fn sum(&self, i: usize) -> T {
            let mut result = self.init_value;
            let mut x = i as i32 - 1;
            while x >= 0 {
                result += self.values[x as usize];
                x = (x & (x + 1)) - 1;
            }
            result
        }

        /// Add a[i] += x
        pub fn add(&mut self, i: usize, x: T) {
            let mut index = i;
            while index < self.values.len() {
                self.values[index] += x;
                index |= index + 1;
            }
        }

        /// Sub a[i] -= x
        pub fn sub(&mut self, i: usize, x: T) {
            let mut index = i;
            while index < self.values.len() {
                self.values[index] -= x;
                index |= index + 1;
            }
        }
        /// Calculate the lower bound of `sum`.
        /// Returns the first index(0-index) so that a[0] + a[1] + a[2] + ... + a[i] >= x
        pub fn lower_bound(&self, x: &T) -> Option<usize> {
            let mut left = 0;
            let mut right = self.values.len();
            let mut result = None;
            while left < right {
                let med = (left + right) / 2;
                match self.sum(med).cmp(x) {
                    std::cmp::Ordering::Less => {
                        left = med + 1;
                    }
                    std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                        result = Some(med - 1);
                        right = med;
                    }
                }
            }
            result
        }

        /// Calculate the upper bound of `sum`.
        /// Returns the first index(0-index) so that a[0] + a[1] + a[2] + ... + a[i] > x
        pub fn upper_bound(&self, x: &T) -> Option<usize> {
            let mut left = 0;
            let mut right = self.values.len();
            let mut result = None;
            while left < right {
                let med = (left + right) / 2;
                match self.sum(med).cmp(x) {
                    std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {
                        left = med + 1;
                    }
                    std::cmp::Ordering::Greater => {
                        result = Some(med - 1);
                        right = med;
                    }
                }
            }
            result
        }
    }
}
#[cfg(test)]
mod test {
    use super::fenwick_tree::FenwickTree;
    use rand::{thread_rng, Rng};
    #[test]
    fn random_array() {
        let n = 1000;
        let mut bit = FenwickTree::new(n, 0);
        let mut v = vec![0; n];
        for i in 0..10000 {
            let value = thread_rng().gen_range(0, 1000);
            let k = thread_rng().gen_range(0, n);
            if i % 2 == 0 {
                v[k] += value;
                bit.add(k, value);
            } else {
                v[k] -= value;
                bit.sub(k, value);
            }

            let mut sum = 0;
            for i in 0..n {
                sum += v[i];
                assert_eq!(sum, bit.sum(i + 1));
            }

            let l = thread_rng().gen_range(0, n);
            let r = thread_rng().gen_range(l, n);
            sum = 0;
            for i in l..r {
                sum += v[i];
                assert_eq!(sum, bit.query(l, i + 1));
            }
        }
    }
}
