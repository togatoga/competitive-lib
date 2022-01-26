use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// Modular Integer
/// NOTE
/// If a modular isn't prime, you can't div.
/// If you want to calculate a combination and permutation, you have to use `mod_comb`.
pub mod mod_int {
    use std::marker::PhantomData;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub struct Mod1000000007;
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub struct Mod998244353;

    pub trait Modulus: Copy + Eq + Copy {
        const VALUE: u32;
    }

    impl Modulus for Mod1000000007 {
        const VALUE: u32 = 1000000007;
    }
    impl Modulus for Mod998244353 {
        const VALUE: u32 = 998244353;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ModInt<
        T: Copy + Clone + Add + AddAssign + Mul + MulAssign + Sub + SubAssign,
        M: Modulus,
    > {
        pub val: T,
        phantom: std::marker::PhantomData<fn() -> M>,
    }
    /// Implementation macros
    #[warn(unused_macros)]
    macro_rules! mod_int_impl {
        ($($t:ty)*) => ($(
            impl <M: Modulus> ModInt<$t, M> {
                pub fn new(x: $t) -> Self {
                    ModInt{val: x.rem_euclid(M::VALUE as $t), phantom: PhantomData}
                }
                pub fn pow(self, e: usize) -> ModInt<$t, M> {
                    let mut result = ModInt::<$t, M>::new(1);
                    let mut cur = self;
                    let mut e = e;
                    while e > 0 {
                        if e & 1 == 1 {
                            result *= cur;
                        }
                        cur *= cur;
                        e >>= 1;
                    }
                    result
                }
            }
            impl <M: Modulus> Add<ModInt<$t, M>> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn add(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    self + rhs.val
                }
            }
            impl <M: Modulus> Add<ModInt<$t, M>> for $t {
                type Output = ModInt<$t, M>;
                fn add(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    let x = self.rem_euclid(M::VALUE as $t);
                    ModInt::<$t, M>::new(x + rhs.val)
                }
            }
            impl <M: Modulus> Add<$t> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn add(self, rhs: $t) -> ModInt<$t, M> {
                    let x = rhs % M::VALUE as $t;
                    ModInt::<$t, M>::new(self.val + x)
                }
            }
            impl <M: Modulus> Sub<ModInt<$t, M>> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn sub(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    self - rhs.val
                }
            }
            impl <M: Modulus> Sub<ModInt<$t, M>> for $t {
                type Output = ModInt<$t, M>;
                fn sub(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    ModInt::<$t, M>::new(self) - rhs
                }
            }
            impl <M: Modulus> Sub<$t> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn sub(self, rhs: $t) -> ModInt<$t, M> {
                    let rhs = rhs.rem_euclid(M::VALUE as $t);
                    let val = (self.val - rhs).rem_euclid(M::VALUE as $t);
                    ModInt::<$t, M>::new(val)
                }
            }


            impl <M: Modulus> AddAssign<ModInt<$t, M>> for ModInt<$t, M> {
                fn add_assign(&mut self, rhs: ModInt<$t, M>) {
                    *self = *self + rhs;
                }
            }
            impl <M: Modulus> AddAssign<$t> for ModInt<$t, M> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }
            impl <M: Modulus> SubAssign<ModInt<$t, M>> for ModInt<$t, M> {
                fn sub_assign(&mut self, rhs: ModInt<$t, M>) {
                    *self = *self - rhs;
                }
            }

            impl <M: Modulus> SubAssign<$t> for ModInt<$t, M> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl <M: Modulus> Div<$t> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn div(self, rhs: $t) -> ModInt<$t, M> {
                    self * ModInt::<$t, M>::new(rhs).pow((M::VALUE - 2) as usize)
                }
            }
            impl <M: Modulus> Div<ModInt<$t, M>> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn div(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    self / rhs.val
                }
            }
            impl <M: Modulus> DivAssign<$t> for ModInt<$t, M> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }
            impl <M: Modulus> DivAssign<ModInt<$t, M>> for ModInt<$t, M> {
                fn div_assign(&mut self, rhs: ModInt<$t, M>) {
                    *self = *self / rhs
                }
            }

            impl <M: Modulus> Mul<ModInt<$t, M>> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn mul(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    self * rhs.val
                }
            }

            impl <M: Modulus> Mul<ModInt<$t, M>> for $t {
                type Output = ModInt<$t, M>;
                fn mul(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    rhs * self
                }
            }

            impl <M: Modulus> Mul<$t> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn mul(self, rhs: $t) -> ModInt<$t, M> {
                    let rhs = rhs.rem_euclid(M::VALUE as $t);
                    ModInt::<$t, M>::new(self.val * rhs)
                }
            }
            impl <M: Modulus> MulAssign<ModInt<$t, M>> for ModInt<$t, M> {
                fn mul_assign(&mut self, rhs: ModInt<$t, M>) {
                    *self = *self * rhs;
                }
            }

            impl <M: Modulus> MulAssign<$t> for ModInt<$t, M> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }
            impl <M: Modulus> Default for ModInt<$t, M> {
                fn default() -> ModInt<$t, M> {
                    ModInt{val: 0, phantom: PhantomData}
                }
            }
            )*)
    }
    mod_int_impl!(usize i64 u64 i128);

    #[allow(dead_code)]
    pub type ModInt1000000007 = ModInt<i64, Mod1000000007>;
    pub type ModInt998244353 = ModInt<i64, Mod998244353>;
}

#[cfg(test)]
mod test {

    use super::mod_int;
    type ModInt = mod_int::ModInt<i64, mod_int::Mod1000000007>;

    #[test]
    fn test_zero() {
        let a = ModInt::new(1_000_000_000);
        let b = ModInt::new(7);
        let c = a + b;

        assert_eq!(c.val, 0);

        let a = ModInt::new(1_000_000_000);
        let c = 7 + a;
        assert_eq!(c.val, 0);

        let a = ModInt::new(2);
        let c = 1 - a;
        assert_eq!(c.val, 1000000006);
    }
    #[test]
    fn test_sub() {
        let a = ModInt::new(10);
        let b = ModInt::new(100);

        assert_eq!((b - a).val, 90);
        assert_eq!((a - b).val, 1_000_000_007 - 90);

        let a = ModInt::new(1);
        assert_eq!((a - -1_000_000_007).val, 1);
    }
    #[test]
    fn test_mul() {
        let a = ModInt::new(1000);
        let b = ModInt::new(1234);
        let c = a * b;
        assert_eq!(c.val, 1234000);

        let a = ModInt::new(500000004);

        let c = 2 * a;
        assert_eq!(c.val, 1);

        let x = ModInt::new(2);
        assert_eq!((-2 * x).val, 1_000_000_003);

        assert_eq!((ModInt::new(100_000) * 100_000).val, 999_999_937);
    }
    #[test]
    fn test_new() {
        let x = ModInt::new((1e9 as i64 + 7) as i64);
        assert_eq!(x.val, 0);
        let x = ModInt::new((1e9 as i64 + 8) as i64);
        assert_eq!(x.val, 1);
        let x = ModInt::new(-1);
        assert_eq!(x.val, 1_000_000_006);
    }

    #[test]
    fn test_div() {
        let x = ModInt::new(12);
        assert_eq!((x / 2).val, 6);
        assert_eq!((x / -2).val, 1_000_000_001);

        assert_eq!((ModInt::new(0) / 1).val, 0);

        assert_eq!((ModInt::new(1) / 42).val, 23_809_524);
    }

    #[test]
    fn test_pow() {
        let a = ModInt::new(2);
        let b = a.pow(10).val;
        assert_eq!(b, 1024);
    }
}
