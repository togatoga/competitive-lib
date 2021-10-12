use cargo_snippet::snippet;
#[snippet(name = "matrix")]
pub mod matrix {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

    pub trait MatrixTrait: Default + Clone + Copy {}
    #[derive(Clone, Default, Debug)]
    pub struct Matrix<T> {
        data: Vec<T>,
        height: usize,
        width: usize,
    }
    impl<T: Default + Copy> Matrix<T> {
        pub fn new(h: usize, w: usize) -> Matrix<T> {
            assert!(h != 0 && w != 0);
            Matrix {
                data: vec![T::default(); h * w],
                height: h,
                width: w,
            }
        }
        pub fn width(&self) -> usize {
            self.width
        }
        pub fn height(&self) -> usize {
            self.height
        }
        pub fn get(&self, y: usize, x: usize) -> &T {
            &self.data[y * self.width + x]
        }
        pub fn get_mut(&mut self, y: usize, x: usize) -> &mut T {
            //cecho!(y, x, self.height, self.width);
            &mut self.data[y * self.width + x]
        }
    }
    impl<T: Default + Copy> From<Vec<Vec<T>>> for Matrix<T> {
        fn from(x: Vec<Vec<T>>) -> Self {
            let h = x.len();
            let w = x[0].len();
            let mut matrix = Matrix::new(h, w);
            for i in 0..h {
                for j in 0..w {
                    *matrix.get_mut(i, j) = x[i][j];
                }
            }
            matrix
        }
    }

    impl<T: MatrixTrait + Mul<Output = T> + AddAssign + MulAssign> Matrix<T> {
        /// A^k
        /// O(hw^2logK)
        pub fn pow(&self, mut k: usize, one: T) -> Self {
            assert!(self.height() == self.width());
            let mut result = Self::new(self.height(), self.width());
            for i in 0..self.height() {
                *result.get_mut(i, i) = one;
            }
            let mut s = self.clone();
            while k > 0 {
                if k & 1 == 1 {
                    result = result.dot(&s);
                }

                s = s.dot(&s);
                k >>= 1;
            }
            result
        }
        /// Matrix A*B = C
        /// (A.height, A.width) = (n1, m1)
        /// (B.height, B.width) = (n2, m2)
        /// (m1 == n2)
        /// multiple A by B and return a new matrix(n1, m2) C = A * B
        /// O(A.height * A.width * B.width)
        pub fn dot(&self, rhs: &Matrix<T>) -> Matrix<T> {
            assert!(self.width() == rhs.height());
            let mut matrix = Matrix::new(self.height(), rhs.width());
            for i in 0..self.height() {
                for k in 0..self.width() {
                    for j in 0..rhs.width() {
                        *matrix.get_mut(i, j) += *self.get(i, k) * *rhs.get(k, j);
                    }
                }
            }
            matrix
        }
        /// A*d
        pub fn mul(&self, d: T) -> Matrix<T> {
            let mut matrix = self.clone();
            matrix.data.iter_mut().for_each(|x| *x *= d);
            matrix
        }
        /// A *= d
        pub fn mul_assign(&mut self, d: T) {
            self.data.iter_mut().for_each(|x| *x *= d);
        }
    }
    impl<T: MatrixTrait + Add<Output = T> + AddAssign> Matrix<T> {
        /// A + B
        pub fn add_mat(&self, rhs: &Matrix<T>) -> Matrix<T> {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            let data: Vec<_> = self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| *x + *y)
                .collect();

            Matrix {
                data,
                height: self.height,
                width: self.width,
            }
        }
        /// A += B
        pub fn add_mat_assign(&mut self, rhs: &Matrix<T>) {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            self.data
                .iter_mut()
                .zip(rhs.data.iter())
                .for_each(|(x, y)| *x += *y);
        }
    }

    impl<T: MatrixTrait + Sub<Output = T> + SubAssign> Matrix<T> {
        /// A - B
        pub fn sub_mat(self, rhs: &Matrix<T>) -> Matrix<T> {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            let data: Vec<_> = self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| *x - *y)
                .collect();

            Matrix {
                data,
                height: self.height,
                width: self.width,
            }
        }
        /// A -= B
        pub fn sub_mat_assign(&mut self, rhs: &Matrix<T>) {
            assert!(self.height() == rhs.height() && self.width() == rhs.width());
            self.data
                .iter_mut()
                .zip(rhs.data.iter())
                .for_each(|(x, y)| *x -= *y);
        }
    }

    /// impl MatrixTrait for * {}
    impl MatrixTrait for i32 {}
    impl MatrixTrait for i64 {}
    impl MatrixTrait for i128 {}
    impl MatrixTrait for u32 {}
    impl MatrixTrait for u64 {}
    impl MatrixTrait for usize {}
}

#[cfg(test)]
mod tests {
    use super::matrix::Matrix;

    #[test]
    fn test_dot_matrix() {
        let a = vec![vec![1, 1], vec![1, 0]];
        let b = vec![vec![5, 2], vec![3, 1]];
        let m1 = Matrix::from(a);
        let m2 = Matrix::from(b);
        let m3 = m1.dot(&m2);
        let c = vec![vec![8, 3], vec![5, 2]];
        for i in 0..m3.height() {
            for j in 0..m3.width() {
                assert_eq!(c[i][j], *m3.get(i, j));
            }
        }

        let a = vec![vec![1, 2]];
        let b = vec![vec![3, 4, 5], vec![6, 7, 8]];
        let m1 = Matrix::from(a);
        let m2 = Matrix::from(b);
        let m3 = m1.dot(&m2);
        let c = vec![vec![15, 18, 21]];
        for i in 0..m3.height() {
            for j in 0..m3.width() {
                assert_eq!(c[i][j], *m3.get(i, j));
            }
        }
    }
}
