pub mod segment_tree {
    pub struct SegmentTree<T, F> {
        seg: Vec<T>,
        n: usize,
        f: F,
        init_value: T,
    }
    impl<T: Copy, F: Fn(T, T) -> T> SegmentTree<T, F> {
        pub fn new(n: usize, init_value: T, f: F) -> SegmentTree<T, F> {
            let mut m = 1;
            while m <= n {
                m <<= 1;
            }
            SegmentTree {
                seg: vec![init_value; 2 * m],
                n: m,
                f: f,
                init_value: init_value,
            }
        }
        pub fn update(&mut self, k: usize, value: T) {
            let mut k = k;
            k += self.n - 1;
            self.seg[k] = value;
            while k > 0 {
                k = (k - 1) >> 1;
                self.seg[k] = (self.f)(self.seg[2 * k + 1], self.seg[2 * k + 2]);
            }
        }
        //Get a query result in the range [left, right)
        pub fn query(&self, left: usize, right: usize) -> T {
            assert!(left < right);
            self.query_range(left, right, 0, 0, self.n)
        }
        fn query_range(
            &self,
            left: usize,
            right: usize,
            k: usize,
            left_bound: usize,
            right_bound: usize,
        ) -> T {
            if right_bound <= left || right <= left_bound {
                self.init_value
            } else if left <= left_bound && right_bound <= right {
                self.seg[k]
            } else {
                let x = self.query_range(
                    left,
                    right,
                    2 * k + 1,
                    left_bound,
                    (left_bound + right_bound) >> 1,
                );
                let y = self.query_range(
                    left,
                    right,
                    2 * k + 2,
                    (left_bound + right_bound) >> 1,
                    right_bound,
                );
                (self.f)(x, y)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::segment_tree::SegmentTree;
    use rand::{thread_rng, Rng};

    #[test]
    fn random_update() {
        let n = 1000;
        //max segment tree
        let mut seg = SegmentTree::new(n, 0, |a, b| std::cmp::max(a, b));
        let mut v = vec![0; n];
        for _ in 0..10000 {
            let value = thread_rng().gen_range(0, 1000);
            let k = thread_rng().gen_range(0, n);
            v[k] = value;
            seg.update(k, value);
            //[0, n)
            assert_eq!(seg.query(0, n), *v.iter().max().unwrap());

            let l = thread_rng().gen_range(0, n);
            let r = thread_rng().gen_range(l, n);
            //two point
            //[l, r + 1)
            let x = v.iter().skip(l).take(r - l + 1).max().unwrap();
            assert_eq!(seg.query(l, r + 1), *x);
        }
    }
}
