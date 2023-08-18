#[allow(clippy::module_inception)]
/// An original code is https://atcoder.jp/contests/abl/submissions/34700239
pub mod lazy_assign_segment_tree {

    const EMPTY_INDEX: u32 = !0;

    /// `LazyAssignSegmentTree`
    #[derive(Debug, Clone)]
    pub struct LazyAssignSegmentTree<T, F> {
        /// The number of node.
        size: usize,
        /// The height of tree.
        height: usize,
        data: Vec<T>,
        /// Stores a doubling value for applying `op`.
        /// Note
        ///     A `table` stores different operations.
        /// table[0] = T1
        /// table[1] = op(table[0], table[0])
        /// table[2] = op(table[1], table[1])
        /// -------------------------------------------
        /// table[3] = T2
        /// table[4] = op(table[3], table[3])
        /// table[5] = op(table[4], table[4])
        table: Vec<T>,
        /// Stores an index for `table`.
        lazy: Vec<u32>,
        n: usize,
        /// |a: i32, b: i32| a + b
        /// |a: i32, b: i32| a * b
        /// |(v1, len1): (i32, i32), (v2, len2): (i32, i32)| (v1 * 10i32.pow(len2) + v2, v1 + v2)
        op: F,
    }

    impl<T, F> LazyAssignSegmentTree<T, F>
    where
        T: Clone,
        F: Fn(&T, &T) -> T,
    {
        /// Makes a new `LazyAssignSegmentTree`
        /// Example
        /// ```
        /// let a = vec![2i64; n];
        /// let mut lazy_seg = LazyAssignSegmentTree::new(a,|a, b| a+b);
        /// ```
        pub fn new(a: &[T], op: F) -> Self {
            assert!(!a.is_empty());
            let size = a.len().next_power_of_two();
            let mut data = vec![a[0].clone(); 2 * size];

            // sets all bottom nodes.
            for (data, a) in data.iter_mut().skip(size).zip(a) {
                *data = a.clone();
            }

            // propagates up to root.
            for i in (1..size).rev() {
                data[i] = op(&data[2 * i], &data[2 * i + 1]);
            }

            let height = size.trailing_zeros() as usize + 1;
            LazyAssignSegmentTree {
                size,
                height,
                data,
                table: vec![],
                lazy: vec![EMPTY_INDEX; 2 * size],
                n: a.len(),
                op,
            }
        }

        /// Returns a `op(data[l], data[l+1], data[l+2], ..., data[r-1])` value.
        pub fn prod(&mut self, l: usize, r: usize) -> T {
            assert!(l < r && r <= self.n);
            self.prod_rec(1, 0, self.size, l, r)
        }

        fn prod_rec(&mut self, v: usize, l: usize, r: usize, x: usize, y: usize) -> T {
            if x <= l && r <= y {
                return self.eval(v);
            }
            self.apply(v);
            let med = (l + r) / 2;
            if x < med && med < y {
                let left = self.prod_rec(2 * v, l, med, x, y);
                let right = self.prod_rec(2 * v + 1, med, r, x, y);
                (self.op)(&left, &right)
            } else if x < med {
                self.prod_rec(2 * v, l, med, x, y)
            } else {
                self.prod_rec(2 * v + 1, med, r, x, y)
            }
        }
        /// Replaces `[l, r)` values `data[l..r]` with a new value `v`.
        pub fn update(&mut self, l: usize, r: usize, v: T) {
            assert!(l < r && r <= self.n);
            let mut t = v.clone();
            for _ in 0..self.height {
                self.table.push(t.clone());
                t = (self.op)(&t, &t);
            }
            // updating recursively
            self.update_rec(1, 0, self.size, l, r, self.table.len() - 1);

            // clear table and apply a stored value to everything.
            if self.table.len() >= self.data.len() {
                for i in 1..self.size {
                    self.apply(i);
                }
                // apply bottom nodes
                for (data, table_index) in self
                    .data
                    .iter_mut()
                    .zip(self.lazy.iter_mut())
                    .skip(self.size)
                    .filter(|(_, table_index)| **table_index != EMPTY_INDEX)
                {
                    *data = self.table[*table_index as usize].clone();
                    *table_index = EMPTY_INDEX;
                }
                self.table.clear();
            }
        }

        fn update_rec(
            &mut self,
            v: usize,
            l: usize,
            r: usize,
            x: usize,
            y: usize,
            table_index: usize,
        ) {
            if x <= l && r <= y {
                self.lazy[v] = table_index as u32;
                return;
            }
            self.apply(v);
            let med = (l + r) / 2;
            if x < med {
                self.update_rec(2 * v, l, med, x, y, table_index - 1);
            }
            if med < y {
                self.update_rec(2 * v + 1, med, r, x, y, table_index - 1);
            }
            self.data[v] = (self.op)(&self.eval(2 * v), &self.eval(2 * v + 1));
        }

        /// Applies a stored value to a node `v` and propagates it's childs.
        fn apply(&mut self, v: usize) {
            let lazy_table_index = self.lazy[v];
            if lazy_table_index != EMPTY_INDEX {
                let old_lazy_table_index = std::mem::replace(&mut self.lazy[v], EMPTY_INDEX);
                assert!(
                    self.lazy[2 * v] == EMPTY_INDEX || self.lazy[2 * v] < old_lazy_table_index - 1
                );
                assert!(
                    self.lazy[2 * v + 1] == EMPTY_INDEX
                        || self.lazy[2 * v + 1] < old_lazy_table_index - 1
                );
                debug_assert!(old_lazy_table_index >= 1);
                self.lazy[2 * v] = old_lazy_table_index - 1;
                self.lazy[2 * v + 1] = old_lazy_table_index - 1;
                self.data[v] = self.table[old_lazy_table_index as usize].clone();
            }
        }
        /// Returns a node `x` value.
        fn eval(&self, x: usize) -> T {
            let lazy_table_index = self.lazy[x];
            if lazy_table_index == EMPTY_INDEX {
                self.data[x].clone()
            } else {
                self.table[lazy_table_index as usize].clone()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};

    use super::lazy_assign_segment_tree::LazyAssignSegmentTree;

    #[test]
    fn test_add() {
        let n = 10;
        let mut a = vec![0; n];
        let mut lazy_seg = LazyAssignSegmentTree::new(&a, |a, b| a + b);
        a.iter_mut().take(5).skip(2).for_each(|x| *x = 5);
        lazy_seg.update(2, 5, 5);
        assert_eq!(lazy_seg.prod(0, n), a.iter().sum::<i32>());
        a.iter_mut().take(9).skip(3).for_each(|x| *x = 7);
        lazy_seg.update(3, 9, 7);
        assert_eq!(lazy_seg.prod(0, n), a.iter().sum::<i32>());
        assert_eq!(lazy_seg.prod(2, 7), a[2..7].iter().sum::<i32>());
    }

    #[test]
    fn random_test() {
        // random test
        let mut rng = thread_rng();
        const INF: i64 = 2000;
        for _ in 0..50 {
            let mut seq = (0..500)
                .map(|_| rng.gen_range(-INF, INF))
                .collect::<Vec<_>>();
            let n = seq.len();
            let mut add_seg = LazyAssignSegmentTree::new(&seq, |a, b| a + b);
            let mut max_seg = LazyAssignSegmentTree::new(&seq, |a, b| *a.max(b));
            for _ in 0..n / 2 {
                let left = rng.gen_range(0, n);
                let right = rng.gen_range(left, n);
                let value = rng.gen_range(-INF, INF);
                seq.iter_mut()
                    .take(right + 1)
                    .skip(left)
                    .for_each(|x| *x = value);
                add_seg.update(left, right + 1, value);
                max_seg.update(left, right + 1, value);
                assert_eq!(
                    add_seg.prod(left, right + 1),
                    seq[left..=right].iter().sum::<i64>()
                );
                assert_eq!(add_seg.prod(0, n), seq.iter().sum::<i64>());
                assert_eq!(max_seg.prod(0, n), *seq.iter().max().unwrap());
                for _ in 0..100 {
                    let l = rng.gen_range(0, n);
                    let r = rng.gen_range(l, n);
                    assert_eq!(add_seg.prod(l, r + 1), seq[l..=r].iter().sum::<i64>());
                    assert_eq!(max_seg.prod(l, r + 1), *seq[l..=r].iter().max().unwrap());
                }
                for (i, x) in seq.iter().enumerate() {
                    assert_eq!(add_seg.prod(i, i + 1), *x);
                    assert_eq!(max_seg.prod(i, i + 1), *x);
                }
            }
        }
    }
}
