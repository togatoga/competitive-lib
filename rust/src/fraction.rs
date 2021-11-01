use cargo_snippet::snippet;
#[snippet]
pub mod fraction {
    /// numerator / denominator
    #[derive(Clone, Copy, Debug)]
    pub struct Fraction {
        numerator: i64,
        denominator: i64,
    }

    // Methods
    impl Fraction {
        /// Make a new Fraction to represent n / d.
        pub fn new(n: i64, d: i64) -> Fraction {
            if d == 0 {
                panic!("Attempted to set denominator to 0");
            }
            if n == 0 {
                return Fraction::zero();
            }

            let (n, d) = if (d > 0 && n > 0) || (d < 0 && n < 0) {
                (n.abs(), d.abs())
            } else {
                (-n.abs(), d.abs())
            };
            Fraction {
                numerator: n,
                denominator: d,
            }
        }

        pub fn zero() -> Fraction {
            Fraction {
                numerator: 1,
                denominator: 0,
            }
        }

        // Getters
        pub fn num(&self) -> i64 {
            self.numerator
        }
        pub fn den(&self) -> i64 {
            self.denominator
        }
        // Reduce method
        pub fn reduce(&self) -> Fraction {
            let mut abs_num = self.num().abs();
            let mut abs_den = self.den().abs();
            let mut gcd = 1;
            // Modified Euclidean algorithm to find greatest common divisor (GCD)
            while (0 != abs_num) && (0 != abs_den) {
                if abs_num > abs_den {
                    gcd = abs_den;
                    abs_num %= abs_den;
                } else {
                    gcd = abs_num;
                    abs_den %= abs_num;
                }
            }
            // Eliminate double negatives or move negative to numerator
            if 0 > self.den() {
                gcd *= -1;
            }
            Fraction {
                numerator: self.num() / gcd,
                denominator: self.den() / gcd,
            }
        }

        // Raising fraction to a power method
        pub fn pow(&self, exp: i64) -> Fraction {
            if exp < 0 {
                // Negative exponents flip the fraction
                let exp_u32 = -exp as u32;
                let f = Fraction {
                    numerator: self.den().pow(exp_u32),
                    denominator: self.num().pow(exp_u32),
                };
                f.reduce()
            } else {
                let exp_u32 = exp as u32;
                let f = Fraction {
                    numerator: self.num().pow(exp_u32),
                    denominator: self.den().pow(exp_u32),
                };
                f.reduce()
            }
        }
    }

    // Arithmetic operations & operator overload
    impl std::ops::Add for Fraction {
        type Output = Fraction;
        fn add(self, other: Fraction) -> Fraction {
            let f = Fraction {
                numerator: self.num() * other.den() + self.den() * other.num(),
                denominator: self.den() * other.den(),
            };
            if f.num() == 0 {
                Fraction::zero()
            } else {
                f.reduce()
            }
        }
    }
    impl std::ops::Sub for Fraction {
        type Output = Fraction;
        fn sub(self, other: Fraction) -> Fraction {
            let f = Fraction {
                numerator: self.num() * other.den() - self.den() * other.num(),
                denominator: self.den() * other.den(),
            };
            if f.num() == 0 {
                Fraction::zero()
            } else {
                f.reduce()
            }
        }
    }
    impl std::ops::Mul for Fraction {
        type Output = Fraction;
        fn mul(self, other: Fraction) -> Fraction {
            let f = Fraction {
                numerator: self.num() * other.num(),
                denominator: self.den() * other.den(),
            };
            f.reduce()
        }
    }
    impl std::ops::Div for Fraction {
        type Output = Fraction;
        fn div(self, other: Fraction) -> Fraction {
            if other.num() == 0 {
                panic!("Attempted to divide by zero.");
            }
            let f = Fraction {
                numerator: self.num() * other.den(),
                denominator: self.den() * other.num(),
            };
            f.reduce()
        }
    }

    // Unary operator overload
    impl std::ops::Neg for Fraction {
        type Output = Fraction;
        fn neg(self) -> Fraction {
            let f = Fraction {
                numerator: -self.num(),
                denominator: self.den(),
            };
            f.reduce()
        }
    }

    // Comparison operator overloads
    use std::cmp::Ordering;
    impl std::cmp::PartialEq for Fraction {
        fn eq(&self, other: &Fraction) -> bool {
            self.num() * other.den() == self.den() * other.num()
        }
    }
    impl std::cmp::Eq for Fraction {}
    impl std::cmp::PartialOrd for Fraction {
        fn partial_cmp(&self, other: &Fraction) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Fraction {
        fn cmp(&self, other: &Fraction) -> Ordering {
            if (0 > self.den()) || (0 > other.den()) {
                // Need to add negative sign to num() and f.num() because
                // negative in denominator causes comparison to be flipped
                (-self.num() * other.den()).cmp(&(-other.num() * self.den()))
            } else {
                (self.num() * other.den()).cmp(&(other.num() * self.den()))
            }
        }
    }

    // Print formatting
    impl std::fmt::Display for Fraction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{}/{}", self.num(), self.den())
        }
    }
}

#[cfg(test)]
mod test {
    use super::fraction::Fraction;
    #[test]
    fn test_fraction() {
        assert!(Fraction::new(1, 2) > Fraction::new(1, 3));
        assert!(Fraction::new(1, 2) == Fraction::new(-1, -2));
        assert!(Fraction::new(1, -2) < Fraction::new(1, 3));

        assert_eq!(Fraction::new(0, 10000), Fraction::zero());
        // mul
        assert_eq!(
            Fraction::new(1, 2) * Fraction::new(2, 3),
            Fraction::new(1, 3)
        );
        // add
        assert_eq!(
            Fraction::new(1, 2) + Fraction::new(2, 3),
            Fraction::new(7, 6)
        );

        // sub
        assert_eq!(Fraction::new(1, 2) - Fraction::new(1, 2), Fraction::zero());

        // div
        assert_eq!(
            Fraction::new(1, 2) / Fraction::new(2, 3),
            Fraction::new(3, 4)
        );
    }
}
