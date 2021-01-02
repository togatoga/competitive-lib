use cargo_snippet::snippet;
#[snippet]
pub mod gcd {
    pub trait Gcd {
        type Output;
        fn gcd(self, other: Self) -> Self::Output;
    }
    #[warn(unused_macros)]
    macro_rules! gcd_unsigned_impl {
        ($($t:ty)*) => ($(
            impl Gcd for $t {
                type Output = $t;
                fn gcd(self, other: Self) -> Self::Output {
                    if self == 0 {
                        other
                    } else if other == 0 {
                        self
                    } else {
                        if self < other {
                            self.gcd(other % self)
                        } else {
                            other.gcd(self % other)
                        }
                    }
                }
            }
        )*)
    }
    #[warn(unused_macros)]
    macro_rules! gcd_signed_impl {
        ($($t:ty)*) => ($(
            impl Gcd for $t {
                type Output = $t;
                fn gcd(self, other: Self) -> Self::Output {
                    if self == 0 {
                        other.abs()
                    } else if other == 0 {
                        self.abs()
                    } else {
                        if self.abs() < other.abs() {
                            self.gcd(other % self)
                        } else {
                            other.gcd(self % other)
                        }
                    }
                }
            }
        )*)
    }
    gcd_signed_impl!(i32 i64);
    gcd_unsigned_impl!(usize u32 u64);
    // A return value is gcd(a, b)
    // ax + by = gcd(a, b)
    // this function sets x and y to satisfiy above formula.
    pub fn extended_gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
        if b == 0 {
            *x = 1;
            *y = 0;
            return a;
        }
        let d = extended_gcd(b, a % b, y, x);
        *y -= a / b * *x;
        return d;
    }

    pub fn module(a: i64, m: i64) -> i64 {
        (a % m + m) % m
    }

    // find a^-1 (a^-1*a == 1 mod m)
    // a and m must be relative primes.
    // But m isn't necessariliy prime
    pub fn inv_module(a: i64, m: i64) -> i64 {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        extended_gcd(a, m, &mut x, &mut y);
        module(x, m)
    }
}

#[cfg(test)]
mod tests {
    use crate::lazy_segment_tree::lazy_segment_tree;

    use super::gcd::*;
    #[test]
    fn test_gcd() {
        assert_eq!(8.gcd(10), 2);
        assert_eq!(10.gcd(8), 2);
        assert_eq!(11.gcd(9), 1);
        assert_eq!(9.gcd(11), 1);
        assert_eq!(10.gcd(100), 10);
        assert_eq!((-10i32).gcd(-100i32), 10);
        assert_eq!((-25i32).gcd(-30i32), 5);
        assert_eq!((-30i32).gcd(-25i32), 5);
    }

    #[test]
    fn test_inv_module() {
        for m in 2..5000 {
            for a in 1..1000 {
                let g = a.gcd(m) as i64;
                let a = a / g;
                let m = m / g;
                if a == 1 || m == 1 {
                    continue;
                }

                let inv_a = inv_module(a, m);
                let r = (a * inv_a) % m;
                dbg!(m, a, inv_a, r);
                assert_eq!(r, 1);
            }
        }
    }
}
