use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]

/// LazySegmentTree is copied from ac-library-rs
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

    /// max(x1, x2, x3, ...)
    pub struct Max<S>(S);
    /// min(x1, x2, x3, ..., xn)
    pub struct Min<S>(S);
    /// x1 + x2 + x3 + ... + xn
    pub struct Additive<S>(S);

    /// x1 *x2 * x3 * ... * xn
    pub struct Multiplicative<S>(S);

    /// Implementation macros
    macro_rules! impl_monoid {
        ($($ty:ty),*) => {
            $(
            impl Monoid for Max<$ty>
            {
                type S = $ty;
                fn identity() -> Self::S {
                    Self::S::min_value()
                }
                fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                    std::cmp::max(*a, *b)
                }
            }
            impl Monoid for Min<$ty>
            {
                type S = $ty;
                fn identity() -> Self::S {
                    Self::S::max_value()
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
    impl_monoid!(i8, i16, i32, i64, u8, u16, u32, usize, u64);

    pub struct LazySegMentTree<F>
    where
        F: MapMonoid,
    {
        n: usize,
        log: usize,
        size: usize,
        d: Vec<<F::M as Monoid>::S>,
        lz: Vec<F::F>,
    }

    impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for LazySegMentTree<F> {
        fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
            let n = v.len();
            let mut log = 0;
            let mut size = 1;
            while size <= n {
                size <<= 1;
                log += 1;
            }

            let mut d = vec![F::identity_element(); 2 * size];
            let lz = vec![F::identity_map(); size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = LazySegMentTree {
                n,
                size,
                log,
                d,
                lz,
            };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }
    impl<F> LazySegMentTree<F>
    where
        F: MapMonoid,
    {
        pub fn new(n: usize) -> Self {
            vec![F::identity_element(); n].into()
        }

        fn update(&mut self, k: usize) {
            self.d[k] = F::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F::F) {
            self.d[k] = F::mapping(&f, &self.d[k]);
            if k < self.size {
                self.lz[k] = F::composition(&f, &self.lz[k]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lz[k].clone());
            self.all_apply(2 * k + 1, self.lz[k].clone());
            self.lz[k] = F::identity_map();
        }

        /// data[p] = x
        /// O(logN)
        pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }
        /// get data[p]
        /// O(logN)
        pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::S {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p].clone()
        }

        /// [l, r)
        /// binary_operation(l,l+1,l+2,...r-1)
        pub fn prod(&mut self, mut l: usize, mut r: usize) -> <F::M as Monoid>::S {
            assert!(l <= r && r <= self.n);
            if l == r {
                return F::identity_element();
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }

            let mut sml = F::identity_element();
            let mut smr = F::identity_element();
            while l < r {
                if l & 1 != 0 {
                    sml = F::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = F::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            F::binary_operation(&sml, &smr)
        }
        /// [l, r)
        /// binary_operation(a[0], ..., a[n - 1])
        pub fn all_prod(&self) -> <F::M as Monoid>::S {
            self.d[1].clone()
        }

        /// data[p] = f(data[p])
        pub fn apply(&mut self, mut p: usize, f: F::F) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = F::mapping(&f, &self.d[p]);
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        /// [l, r)
        /// data[p] = f(data[p]) p=l,l+1,...r-1
        pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: F::F) {
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }

            {
                let l2 = l;
                let r2 = r;
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
                l = l2;
                r = r2;
            }

            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }
    }

    use std::fmt::{Debug, Error, Formatter};

    impl<F> Debug for LazySegMentTree<F>
    where
        F: MapMonoid,
        F::F: Debug,
        <F::M as Monoid>::S: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            for i in 0..self.log {
                f.write_fmt(format_args!("{:?}\t", self.d[self.log + i]))?;
            }
            Ok(())
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

        (0..100).for_each(|_| {
            let left = rng.gen_range(0, n);
            let right = rng.gen_range(left, n) + 1;
            let value = rng.gen_range(0, 100);
            for i in left..right {
                seq[i] += value;
            }
            let seq_max = *seq.iter().skip(left).take(right - left).max().unwrap();
            seg.apply_range(left, right, value);
            let seg_max = seg.prod(left, right);
            assert_eq!(seq_max, seg_max);
            for i in left..right {
                assert_eq!(seg.prod(i, i + 1), seq[i]);
            }
        });
    }

    use super::super::mod_int::mod_int;
    type ModInt = mod_int::ModInt<i64, mod_int::Mod1000000007>;
    struct AdditiveMulMod;
    impl Monoid for Additive<ModInt> {
        type S = ModInt;
        fn identity() -> Self::S {
            ModInt::new(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    impl MapMonoid for AdditiveMulMod {
        type M = Additive<ModInt>;
        type F = i64;
        fn identity_map() -> Self::F {
            1
        }
        fn mapping(&f: &Self::F, &x: &ModInt) -> ModInt {
            x * f
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f * g
        }
    }

    #[test]
    fn test_additive_mul_mod() {
        let mut rng = thread_rng();
        let mut seq: Vec<ModInt> = (0..1000)
            .map(|_| rng.gen_range(0, 1000))
            .map(ModInt::new)
            .collect();
        let n = seq.len();
        let mut seg: LazySegMentTree<AdditiveMulMod> = LazySegMentTree::from(seq.clone());

        (0..100).for_each(|_| {
            let left = rng.gen_range(0, n);
            let right = rng.gen_range(left, n) + 1;
            let value = rng.gen_range(0, 100);
            for i in left..right {
                seq[i] *= value;
            }
            let seq_total_mod = seq
                .iter()
                .skip(left)
                .take(right - left)
                .fold(ModInt::new(0), |x, y| x + *y);
            seg.apply_range(left, right, value);
            let seg_total_mod = seg.prod(left, right);
            assert_eq!(seq_total_mod, seg_total_mod);
            for i in left..right {
                assert_eq!(seg.prod(i, i + 1), seq[i]);
            }
        });
    }
}
