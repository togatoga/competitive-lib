pub mod mod_int {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
    type Num = usize;
    //we may need to change this value
    const MOD: Num = 1e9 as Num + 7;
    #[derive(Clone, Copy, Debug)]
    pub struct ModInt<T: Copy + Clone>(pub T);

    impl ModInt<Num> {
        pub fn new(x: Num) -> Self {
            ModInt(x)
        }
        pub fn pow(self, e: usize) -> ModInt<Num> {
            let mut result = ModInt::new(1);
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

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.0
        }
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: Num) -> ModInt<Num> {
            let mut tmp = rhs + self.0;
            if tmp >= self::MOD {
                tmp = tmp - self::MOD;
            }
            ModInt(tmp)
        }
    }
    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.0
        }
    }
    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.0 < rhs { self.0 + MOD } else { self.0 };
            ModInt(value - rhs)
        }
    }
    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self + rhs;
        }
    }
    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, rhs: Num) {
            *self = *self + rhs;
        }
    }
    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self - rhs;
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, rhs: Num) {
            *self = *self - rhs;
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.0
        }
    }
    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: Num) -> ModInt<Num> {
            ModInt(self.0 * rhs % self::MOD)
        }
    }
    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs;
        }
    }

    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::mod_int::*;
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
    fn test_pow() {
        let a = ModInt::new(2);
        let b = a.pow(10).0;
        assert_eq!(b, 1024);
    }
}
