pub mod lazy_segment_tree {
    pub trait Monoid {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
    }
    pub trait MapMonid {
        type M: Monoid;
        type F: Clone;

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

    pub struct LazySegMentTree<F>
    where
        F: MapMonid,
    {
        size: usize,
        height: usize,
        data: Vec<<F::M as Monoid>::S>,
        lazy: Vec<F::F>,
    }
    impl<F> LazySegMentTree<F>
    where
        F: MapMonid,
    {
        pub fn new(n: usize) -> Self {
            let mut size = 1;
            let mut height = 0;
            while size < n {
                size <<= 1;
                height += 1;
            }
            let data = vec![F::identity_element(); 2 * size];
            let lazy = vec![F::identity_map(); 2 * size];
            LazySegMentTree {
                size,
                height,
                data,
                lazy,
            }
        }
        pub fn set(&mut self, p: usize, x: <F::M as Monoid>::S) {
            self.data[p + self.size] = x;
        }

        fn eval(&mut self, k: usize) {
            if k < self.size {
                self.lazy[2 * k] = F::composition(&self.lazy[2 * k], &self.lazy[k]);
                self.lazy[2 * k + 1] = F::composition(&self.lazy[2 * k + 1], &self.lazy[k]);
            }
            self.data[k] = F::mapping(&self.lazy[k], &self.data[k]);
        }
        fn update(&mut self, a: usize, b: usize, f: F::F, k: usize, l: usize, r: usize) {
            self.eval(k);
            if a <= l && r <= b {
                self.lazy[k] = F::composition(&self.lazy[k], &f);
                self.eval(k);
            } else if a < r && l < b {
                self.update(a, b, f.clone(), 2 * k, l, (l + r) >> 1);
                self.update(a, b, f.clone(), 2 * k + 1, (l + r) >> 1, r);
                self.data[k] = F::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
            }
        }
        pub fn apply(&mut self, a: usize, b: usize, f: F::F) {
            self.update(a, b, f, 1, 0, self.size);
        }
    }
}
