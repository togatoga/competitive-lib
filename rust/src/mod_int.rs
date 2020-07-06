pub mod mod_int {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    //we may need to change this value
    const MOD: i64 = 1e9 as i64 + 7;

    #[derive(Clone, Copy, Debug)]
    pub struct ModInt<T: Copy + Clone + Add + AddAssign + Mul + MulAssign + Sub + SubAssign>(pub T);

    #[warn(unused_macros)]
    macro_rules! mod_int_impl {
        ($($t:ty)*) => ($(
            impl Add<ModInt<$t>> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn add(self, other: ModInt<$t>) -> ModInt<$t> {
                    self + other.0
                }
            }
            impl Add<$t> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn add(self, rhs: $t) -> ModInt<$t> {
                    let mut tmp = rhs + self.0;
                    if tmp >= self::MOD as $t {
                        tmp = tmp - self::MOD as $t;
                    }
                    ModInt(tmp)
                }
            }
            impl Sub<ModInt<$t>> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn sub(self, rhs: ModInt<$t>) -> ModInt<$t> {
                    self - rhs.0
                }
            }
            impl Sub<$t> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn sub(self, rhs: $t) -> ModInt<$t> {
                    let rhs = if rhs >= self::MOD as $t { rhs % self::MOD as $t} else { rhs };
                    let value = if self.0 < rhs { self.0 + self::MOD as $t} else { self.0 };
                    ModInt(value - rhs)
                }
            }
            impl AddAssign<ModInt<$t>> for ModInt<$t> {
                fn add_assign(&mut self, rhs: ModInt<$t>) {
                    *self = *self + rhs;
                }
            }
            impl AddAssign<$t> for ModInt<$t> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }
            impl SubAssign<ModInt<$t>> for ModInt<$t> {
                fn sub_assign(&mut self, rhs: ModInt<$t>) {
                    *self = *self - rhs;
                }
            }

            impl SubAssign<$t> for ModInt<$t> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl Div<$t> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn div(self, mut rhs: $t) -> ModInt<$t> {
                    if rhs >= MOD as $t {
                        rhs %= MOD as $t;
                    }
                    self * ModInt(rhs).pow((MOD - 2) as usize)
                }
            }
            impl Div<ModInt<$t>> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn div(self, rhs: ModInt<$t>) -> ModInt<$t> {
                    self / rhs.0
                }
            }
            impl DivAssign<$t> for ModInt<$t> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs
                }
            }
            impl DivAssign<ModInt<$t>> for ModInt<$t> {
                fn div_assign(&mut self, rhs: ModInt<$t>) {
                    *self = *self / rhs
                }
            }

            impl Mul<ModInt<$t>> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn mul(self, rhs: ModInt<$t>) -> ModInt<$t> {
                    self * rhs.0
                }
            }
            impl Mul<$t> for ModInt<$t> {
                type Output = ModInt<$t>;
                fn mul(self, rhs: $t) -> ModInt<$t> {
                    ModInt(self.0 * (rhs % self::MOD as $t) % self::MOD as $t)
                }
            }
            impl MulAssign<ModInt<$t>> for ModInt<$t> {
                fn mul_assign(&mut self, rhs: ModInt<$t>) {
                    *self = *self * rhs;
                }
            }

            impl MulAssign<$t> for ModInt<$t> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
                }
            }
            impl ModInt<$t> {
                pub fn new(x: $t) -> Self {
                    ModInt(x % self::MOD as $t)
                }
                pub fn pow(self, e: usize) -> ModInt<$t> {
                    let mut result = ModInt::<$t>::new(1);
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
    type ModInt = mod_int::ModInt<usize>;

    #[test]
    fn test_zero() {
        let a = ModInt::new(1_000_000_000);
        let b = ModInt::new(7);
        let c = a + b;

        assert_eq!(c.0, 0);
    }

    #[test]
    fn test_mul() {
        let a = ModInt::new(1000);
        let b = ModInt::new(1234);
        let c = a * b;
        assert_eq!(c.0, 1234000);
    }
    #[test]
    fn test_new() {
        let x = ModInt::new((1e9 as i64 + 7) as usize);
        assert_eq!(x.0, 0);
        let x = ModInt::new((1e9 as i64 + 8) as usize);
        assert_eq!(x.0, 1);
    }

    #[test]
    fn test_pow() {
        let a = ModInt::new(2);
        let b = a.pow(10).0;
        assert_eq!(b, 1024);
    }
}
