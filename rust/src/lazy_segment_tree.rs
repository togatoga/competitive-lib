pub mod lazy_segment_tree {
    pub trait Monoid {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
    }
    pub trait MapMonoid {
        type M: Monoid;
        type F: Clone + PartialEq;

        fn identity_element() -> <Self::M as Monoid>::S {
            Self::M::identity()
        }

        fn binary_operation(
            a: &<Self::M as Monoid>::S,
            b: &<Self::M as Monoid>::S,
        ) -> <Self::M as Monoid>::S {
            Self::M::binary_operation(a, b)
        }

        fn identity_map() -> Self::F;
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }

    pub struct Max<S>(S);
    pub struct Min<S>(S);
    pub struct Additive<S>(S);
    pub struct Multiplicative<S>(S);

    macro_rules! impl_monoid {
        ($($ty:ty),*) => {
            $(
            impl Monoid for Max<$ty>
            {
                type S = $ty;
                fn identity() -> Self::S {
                    Self::S::MIN
                }
                fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                    std::cmp::max(*a, *b)
                }
            }
            impl Monoid for Min<$ty>
            {
                type S = $ty;
                fn identity() -> Self::S {
                    Self::S::MAX
                }
                fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                    std::cmp::min(*a, *b)
                }
            }
            impl Monoid for Additive<$ty>
            {
                type S = $ty;
                fn identity() -> Self::S {
                    0
                }
                fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                    *a + *b
                }
            }
            impl Monoid for Multiplicative<$ty>
            {
                type S = $ty;
                fn identity() -> Self::S {
                    1
                }
                fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                    *a * *b
                }
            }
        )*
        };
    }
    impl_monoid!(i8, i16, i32, i64, u8, u16, u32, u64, usize);

    pub struct LazySegMentTree<F>
    where
        F: MapMonoid,
    {
        size: usize,
        data: Vec<<F::M as Monoid>::S>,
        lazy: Vec<F::F>,
    }
    impl<F> LazySegMentTree<F>
    where
        F: MapMonoid,
    {
        pub fn new(n: usize) -> Self {
            let mut size = 1;
            while size < n {
                size <<= 1;
            }
            let data = vec![F::identity_element(); 2 * size];
            let lazy = vec![F::identity_map(); 2 * size];
            LazySegMentTree { size, data, lazy }
        }

        //0-index
        pub fn set(&mut self, p: usize, x: <F::M as Monoid>::S) {
            self.data[p + self.size] = x;
        }

        //Must call this function just after finishing constructing the segment tree
        pub fn build(&mut self) {
            for k in (1..self.size).rev() {
                self.data[k] = F::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
            }
        }

        //propagate lazy values to childs
        fn eval(&mut self, k: usize) {
            if self.lazy[k] == F::identity_map() {
                return;
            }
            if k < self.size {
                self.lazy[2 * k] = F::composition(&self.lazy[2 * k], &self.lazy[k]);
                self.lazy[2 * k + 1] = F::composition(&self.lazy[2 * k + 1], &self.lazy[k]);
            }
            self.data[k] = F::mapping(&self.lazy[k], &self.data[k]);
            self.lazy[k] = F::identity_map();
        }

        fn apply_internal(&mut self, a: usize, b: usize, f: F::F, k: usize, l: usize, r: usize) {
            self.eval(k);
            if a <= l && r <= b {
                self.lazy[k] = F::composition(&self.lazy[k], &f);
                self.eval(k);
            } else if a < r && l < b {
                self.apply_internal(a, b, f.clone(), 2 * k, l, (l + r) >> 1);
                self.apply_internal(a, b, f.clone(), 2 * k + 1, (l + r) >> 1, r);
                self.data[k] = F::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
            }
        }

        //0-index [a, b)
        //a=f(a),a+1=f(a+1),...,b-1=f(b-1)
        //e.g f(x) = x + 1
        //a=a+1,a+1=a+2,...,b-1=b
        pub fn apply(&mut self, a: usize, b: usize, f: F::F) {
            self.apply_internal(a, b, f, 1, 0, self.size);
        }

        fn get_internal(
            &mut self,
            a: usize,
            b: usize,
            k: usize,
            l: usize,
            r: usize,
        ) -> <F::M as Monoid>::S {
            self.eval(k);
            if a <= l && r <= b {
                self.data[k].clone()
            } else if a < r && l < b {
                F::binary_operation(
                    &self.get_internal(a, b, 2 * k, l, (l + r) >> 1),
                    &self.get_internal(a, b, 2 * k + 1, (l + r) >> 1, r),
                )
            } else {
                F::identity_element()
            }
        }

        //0-index [a, b)
        //g(a.a+1...b-1)
        //e.g g(x, y) = min(x, y)
        pub fn get(&mut self, a: usize, b: usize) -> <F::M as Monoid>::S {
            self.get_internal(a, b, 1, 0, self.size)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lazy_segment_tree::*;
    use rand::{thread_rng, Rng};
    struct MaxAdd;
    impl MapMonoid for MaxAdd {
        type M = Max<i32>;
        type F = i32;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(&f: &i32, &x: &i32) -> i32 {
            f + x
        }

        fn composition(&f: &i32, &g: &i32) -> i32 {
            f + g
        }
    }

    #[test]
    fn test_max_add() {
        let mut rng = thread_rng();
        let mut seq: Vec<i32> = (0..1000).map(|_| rng.gen_range(0, 1000)).collect();
        let n = seq.len();
        let mut seg: LazySegMentTree<MaxAdd> = LazySegMentTree::new(n);
        for (i, x) in seq.iter().enumerate() {
            seg.set(i, *x);
        }
        seg.build();

        (0..100).for_each(|_| {
            let left = rng.gen_range(0, n);
            let right = rng.gen_range(left, n) + 1;
            let value = rng.gen_range(0, 100);
            for i in left..right {
                seq[i] += value;
            }
            let seq_max = *seq.iter().skip(left).take(right - left).max().unwrap();
            seg.apply(left, right, value);
            let seg_max = seg.get(left, right);
            for i in left..right {
                assert_eq!(seg.get(i, i + 1), seq[i]);
            }
            assert_eq!(seq_max, seg_max);
        });
    }
}
