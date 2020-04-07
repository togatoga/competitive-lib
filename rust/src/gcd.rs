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
}

#[cfg(test)]
mod tests {
    use super::gcd::Gcd;
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
}
