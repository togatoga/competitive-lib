use cargo_snippet::snippet;
#[snippet(name = "matrix")]
pub mod matrix {
    use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
    pub trait MatrixTrait: Add + Sub + Mul + Copy + Clone + Default {}
    #[derive(Clone)]
    pub struct Matrix<T> {
        data: Vec<Vec<T>>,
    }

    impl<T: MatrixTrait> Matrix<T> {
        pub fn new(h: usize, w: usize) -> Matrix<T> {
            assert!(h != 0 && w != 0);
            Matrix {
                data: vec![vec![T::default(); w]; h],
            }
        }
        pub fn width(&self) -> usize {
            self.data[0].len()
        }
        pub fn height(&self) -> usize {
            self.data.len()
        }
    }
    impl<T: MatrixTrait + Mul<Output = T> + AddAssign> Matrix<T> {
        /// O(hw^2logK)
        pub fn pow(&self, mut k: usize, one: T) -> Self {
            assert!(self.height() == self.width());
            let mut result = Self::new(self.height(), self.width());
            for i in 0..self.height() {
                result[i][i] = one;
            }
            let mut s = self.clone();
            while k > 0 {
                if k & 1 == 1 {
                    result = result * s.clone();
                }
                s = s.clone() * s;
                k >>= 1;
            }
            result
        }
    }

    impl<T: MatrixTrait + Add<Output = T>> Add<&Matrix<T>> for Matrix<T> {
        type Output = Matrix<T>;
        fn add(self, rhs: &Matrix<T>) -> Self::Output {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            let mut data = vec![vec![T::default(); self.width()]; self.height()];
            for i in 0..self.height() {
                for j in 0..self.width() {
                    data[i][j] = self.data[i][j] + rhs.data[i][j];
                }
            }
            Matrix { data }
        }
    }

    impl<T: MatrixTrait + AddAssign> AddAssign<&Matrix<T>> for Matrix<T> {
        fn add_assign(&mut self, rhs: &Matrix<T>) {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            for i in 0..self.height() {
                for j in 0..self.width() {
                    self.data[i][j] += rhs.data[i][j];
                }
            }
        }
    }

    impl<T: MatrixTrait + Sub<Output = T>> Sub<&Matrix<T>> for Matrix<T> {
        type Output = Matrix<T>;
        fn sub(self, rhs: &Matrix<T>) -> Self::Output {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            let mut data = vec![vec![T::default(); self.width()]; self.height()];
            for i in 0..self.height() {
                for j in 0..self.width() {
                    data[i][j] = self.data[i][j] - rhs.data[i][j];
                }
            }
            Matrix { data }
        }
    }

    impl<T: MatrixTrait + SubAssign> SubAssign<&Matrix<T>> for Matrix<T> {
        fn sub_assign(&mut self, rhs: &Matrix<T>) {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            for i in 0..self.height() {
                for j in 0..self.width() {
                    self.data[i][j] -= rhs.data[i][j];
                }
            }
        }
    }

    impl<T: MatrixTrait> Index<usize> for Matrix<T> {
        type Output = Vec<T>;
        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }
    impl<T: MatrixTrait> IndexMut<usize> for Matrix<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    impl<T: MatrixTrait + Mul<Output = T> + AddAssign> Mul<&Matrix<T>> for Matrix<T> {
        type Output = Matrix<T>;
        /// O(h1w1w2)
        fn mul(self, rhs: &Matrix<T>) -> Self::Output {
            assert!(self.width() == rhs.height());
            let mut data = vec![vec![T::default(); rhs.width()]; self.height()];
            for i in 0..self.height() {
                for k in 0..self.width() {
                    for j in 0..rhs.width() {
                        data[i][j] += self[i][k] * rhs[k][j];
                    }
                }
            }
            Matrix { data }
        }
    }
    impl<T: MatrixTrait + Mul<Output = T> + AddAssign> Mul<Matrix<T>> for Matrix<T> {
        type Output = Matrix<T>;
        /// O(h1w1w2)
        fn mul(self, rhs: Matrix<T>) -> Self::Output {
            assert!(self.width() == rhs.height());
            let mut data = vec![vec![T::default(); rhs.width()]; self.height()];
            for i in 0..self.height() {
                for k in 0..self.width() {
                    for j in 0..rhs.width() {
                        data[i][j] += self[i][k] * rhs[k][j];
                    }
                }
            }
            Matrix { data }
        }
    }

    impl<T: MatrixTrait + Mul<Output = T> + MulAssign> Mul<T> for Matrix<T> {
        type Output = Matrix<T>;
        fn mul(self, rhs: T) -> Self::Output {
            let mut data = self.data.clone();
            for i in 0..self.height() {
                for j in 0..self.width() {
                    data[i][j] *= rhs;
                }
            }
            Matrix { data }
        }
    }
}
