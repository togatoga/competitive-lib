pub mod fenwick_tree {
    //Binary Indexed Tree
    use std::ops::*;
    pub struct FenwickTree<T> {
        values: Vec<T>,
        init_value: T,
    }

    impl<T: Copy + Clone + AddAssign + Sub<Output = T>> FenwickTree<T> {
        pub fn new(n: usize, init_value: &T) -> FenwickTree<T> {
            FenwickTree {
                values: vec![init_value.clone(); n + 1],
                init_value: init_value.clone(),
            }
        }
        //[l, r)
        pub fn query(&self, l: usize, r: usize) -> T {
            self.sum(r) - self.sum(l)
        }
        //[0, i)
        pub fn sum(&self, i: usize) -> T {
            let mut result = self.init_value;
            let mut x = i as i32 - 1;
            while x >= 0 {
                result += self.values[x as usize];
                x = (x & (x + 1)) - 1;
            }
            result
        }

        pub fn add(&mut self, i: usize, x: T) {
            let mut index = i;
            while index < self.values.len() {
                self.values[index] += x;
                index |= index + 1;
            }
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
        let mut bit = FenwickTree::new(n, &0);
        let mut v = vec![0; n];
        for _ in 0..10000 {
            let value = thread_rng().gen_range(0, 1000);
            let k = thread_rng().gen_range(0, n);
            v[k] += value;
            bit.add(k, value);

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
