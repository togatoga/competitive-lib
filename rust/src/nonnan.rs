use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod nonnan {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    pub trait NonNanValue:
        PartialOrd
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Copy
        + Sized
    {
    }
    impl NonNanValue for f64 {}
    impl NonNanValue for f32 {}
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct NonNan<T: NonNanValue>(pub T);
    impl<T: NonNanValue> NonNan<T> {
        pub fn new(x: T) -> NonNan<T> {
            NonNan(x)
        }
    }
    impl<T: NonNanValue> From<T> for NonNan<T> {
        fn from(from: T) -> NonNan<T> {
            NonNan::new(from)
        }
    }

    impl<T: NonNanValue> PartialOrd for NonNan<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: NonNanValue> Eq for NonNan<T> {}
    impl<T: NonNanValue> Ord for NonNan<T> {
        fn cmp(&self, other: &NonNan<T>) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).unwrap()
        }
    }

    impl<T: NonNanValue> Add<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn add(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::new(self.0 + rhs.0)
        }
    }

    impl<T: NonNanValue> Add<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn add(self, rhs: T) -> NonNan<T> {
            NonNan::new(self.0 + rhs)
        }
    }

    impl<T: NonNanValue> AddAssign<NonNan<T>> for NonNan<T> {
        fn add_assign(&mut self, rhs: NonNan<T>) {
            *self = *self + rhs
        }
    }

    impl<T: NonNanValue> AddAssign<T> for NonNan<T> {
        fn add_assign(&mut self, rhs: T) {
            *self = *self + rhs
        }
    }

    impl<T: NonNanValue> Sub<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn sub(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::new(self.0 - rhs.0)
        }
    }

    impl<T: NonNanValue> Sub<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn sub(self, rhs: T) -> NonNan<T> {
            NonNan::new(self.0 - rhs)
        }
    }

    impl<T: NonNanValue> SubAssign<NonNan<T>> for NonNan<T> {
        fn sub_assign(&mut self, rhs: NonNan<T>) {
            *self = *self - rhs;
        }
    }

    impl<T: NonNanValue> SubAssign<T> for NonNan<T> {
        fn sub_assign(&mut self, rhs: T) {
            *self = *self - rhs;
        }
    }

    impl<T: NonNanValue> Mul<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn mul(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::from(self.0 * rhs.0)
        }
    }

    impl<T: NonNanValue> Mul<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn mul(self, rhs: T) -> NonNan<T> {
            NonNan::from(self.0 * rhs)
        }
    }

    impl<T: NonNanValue> MulAssign<NonNan<T>> for NonNan<T> {
        fn mul_assign(&mut self, rhs: NonNan<T>) {
            *self = *self * rhs
        }
    }

    impl<T: NonNanValue> MulAssign<T> for NonNan<T> {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs
        }
    }

    impl<T: NonNanValue> Div<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn div(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::new(self.0 / rhs.0)
        }
    }

    impl<T: NonNanValue> Div<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn div(self, rhs: T) -> NonNan<T> {
            NonNan::new(self.0 / rhs)
        }
    }

    impl<T: NonNanValue> DivAssign<NonNan<T>> for NonNan<T> {
        fn div_assign(&mut self, rhs: NonNan<T>) {
            *self = *self / rhs
        }
    }

    impl<T: NonNanValue> DivAssign<T> for NonNan<T> {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::nonnan::NonNan;
    #[allow(clippy::float_cmp)]
    #[test]
    fn test_nonanf64() {
        let mut f = NonNan::from(1.0);

        assert_eq!(f.0, 1.0);
        assert_eq!(f + 1.0, NonNan::new(2.0));
        f += 1.0;
        assert_eq!(f.0, 2.0);
        assert_eq!(f - 1.0, NonNan::new(1.0));
        f -= 2.0;
        assert_eq!(f.0, 0.0);

        f = NonNan::from(2.0);
        assert_eq!(f * 3.0, NonNan::new(6.0));
        assert_eq!(f / 4.0, NonNan::new(0.5));
        f *= 3.0;
        assert_eq!(f, NonNan::new(6.0));
        f /= 12.0;
        assert_eq!(f, NonNan::new(0.5));

        let mut values = vec![1.0, 12345.67890, -10000.0, 1e10, 0.0]
            .into_iter()
            .map(NonNan)
            .collect::<Vec<_>>();
        values.sort();
        assert_eq!(values[0], NonNan::new(-10000.0));
        assert_eq!(values[1], NonNan::new(0.0));
        assert_eq!(values[2], NonNan::new(1.0));
        assert_eq!(values[3], NonNan::new(12345.67890));
        assert_eq!(values[4], NonNan::new(1e10));
    }
}
