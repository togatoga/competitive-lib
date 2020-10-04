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

    #[derive(Clone, Copy, Debug)]
    pub struct ModInt<
        T: Copy + Clone + Add + AddAssign + Mul + MulAssign + Sub + SubAssign,
        M: Modulus,
    > {
        pub val: T,
        phantom: std::marker::PhantomData<fn() -> M>,
    }

    #[warn(unused_macros)]
    macro_rules! mod_int_impl {
        ($($t:ty)*) => ($(
            impl <M: Modulus> Add<ModInt<$t, M>> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn add(self, other: ModInt<$t, M>) -> ModInt<$t, M> {
                    self + other.val
                }
            }
            impl <M: Modulus> Add<$t> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn add(self, rhs: $t) -> ModInt<$t, M> {
                    let mut val = rhs + self.val;
                    if val >= M::VALUE as $t {
                        val = val - M::VALUE as $t;
                    }
                    ModInt {val, phantom: PhantomData}
                }
            }
            impl <M: Modulus> Sub<ModInt<$t, M>> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn sub(self, rhs: ModInt<$t, M>) -> ModInt<$t, M> {
                    self - rhs.val
                }
            }
            impl <M: Modulus> Sub<$t> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn sub(self, rhs: $t) -> ModInt<$t, M> {
                    let rhs = if rhs >= M::VALUE as $t { rhs % M::VALUE as $t} else { rhs };
                    let val = if self.val < rhs { self.val + M::VALUE as $t} else { self.val };
                    ModInt {val, phantom: PhantomData}
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
                fn div(self, mut rhs: $t) -> ModInt<$t, M> {
                    if rhs >= M::VALUE as $t {
                        rhs %= M::VALUE as $t;
                    }
                    self * ModInt {val: rhs, phantom: PhantomData}.pow((M::VALUE - 2) as usize)
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
            impl <M: Modulus> Mul<$t> for ModInt<$t, M> {
                type Output = ModInt<$t, M>;
                fn mul(self, rhs: $t) -> ModInt<$t, M> {
                    ModInt {val: self.val * (rhs % M::VALUE as $t) % M::VALUE as $t, phantom: PhantomData}
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
            impl <M: Modulus> ModInt<$t, M> {
                pub fn new(x: $t) -> Self {
                    ModInt{val: x % M::VALUE as $t, phantom: PhantomData}
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
            )*)
    }
    mod_int_impl!(usize i32 i64 u32 u64);
}

#[cfg(test)]
mod test {
    use super::mod_int;
    type ModInt = mod_int::ModInt<usize, mod_int::Mod1000000007>;

    #[test]
    fn test_zero() {
        let a = ModInt::new(1_000_000_000);
        let b = ModInt::new(7);
        let c = a + b;

        assert_eq!(c.val, 0);
    }

    #[test]
    fn test_mul() {
        let a = ModInt::new(1000);
        let b = ModInt::new(1234);
        let c = a * b;
        assert_eq!(c.val, 1234000);
    }
    #[test]
    fn test_new() {
        let x = ModInt::new((1e9 as i64 + 7) as usize);
        assert_eq!(x.val, 0);
        let x = ModInt::new((1e9 as i64 + 8) as usize);
        assert_eq!(x.val, 1);
    }

    #[test]
    fn test_pow() {
        let a = ModInt::new(2);
        let b = a.pow(10).val;
        assert_eq!(b, 1024);
    }
}
