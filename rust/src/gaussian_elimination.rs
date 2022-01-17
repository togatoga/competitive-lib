use cargo_snippet::snippet;
#[snippet]
#[allow(clippy::module_inception, clippy::many_single_char_names)]
/// Gaussian elimination
/// AC: https://atcoder.jp/contests/typical90/submissions/23197872
pub mod gaussian_elimination {
    use std::{
        mem::swap,
        ops::{Index, IndexMut},
    };

    type T = i32;
    #[derive(Debug, Default)]
    pub struct Matrix {
        row: usize,
        col: usize,
        value: Vec<Vec<T>>,
    }
    impl Matrix {
        pub fn new(row: usize, col: usize) -> Matrix {
            Matrix {
                row,
                col,
                value: vec![vec![T::default(); col]; row],
            }
        }
        fn swap(&mut self, i: usize, j: usize) {
            self.value.swap(i, j);
        }
    }
    impl From<Vec<Vec<T>>> for Matrix {
        fn from(matrix: Vec<Vec<T>>) -> Self {
            Matrix {
                row: matrix.len(),
                col: matrix[0].len(),
                value: matrix,
            }
        }
    }
    impl Index<usize> for Matrix {
        type Output = [T];
        fn index(&self, index: usize) -> &Self::Output {
            &self.value[index]
        }
    }
    impl IndexMut<usize> for Matrix {
        fn index_mut(&mut self, index: usize) -> &mut [T] {
            &mut self.value[index]
        }
    }

    /// m x n matrix
    /// O(N^3)
    /// A return value is a rank of a given matrix
    pub fn eliminate(matrix: &mut Matrix, is_extended: bool) -> usize {
        let (m, n) = (matrix.row, matrix.col);
        let mut rank = 0;
        for col in 0..n {
            // Skip the last row if a matrix is extended.
            if is_extended && col == n - 1 {
                break;
            }
            let mut pivot = None;
            for row in rank..m {
                if matrix[row][col] != 0 {
                    pivot = Some(row);
                    break;
                }
            }
            if let Some(pivot) = pivot {
                // swap the row
                matrix.swap(pivot, rank);
                let fac = matrix[rank][col];
                assert_ne!(fac, 0);
                for x in 0..n {
                    matrix[rank][x] /= fac;
                }

                // eleminate all pivot's col values(make all values 0)
                for y in 0..m {
                    if y != rank && matrix[y][col] != 0 {
                        let fac = matrix[y][col];
                        for x in 0..n {
                            matrix[y][x] -= matrix[rank][x] * fac;
                        }
                    }
                }
                rank += 1;
            }
        }
        rank
    }

    /// x^-1 (mod module)
    fn inv_module(mut a: i64, module: i64) -> T {
        let mut b = module;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            swap(&mut a, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= module;
        if u < 0 {
            u += module;
        }
        u as T
    }
    pub fn eliminate_mod(matrix: &mut Matrix, is_extended: bool, module: T) -> usize {
        let (m, n) = (matrix.row, matrix.col);
        let mut rank = 0;
        for col in 0..n {
            // Skip the last row if a matrix is extended.
            if is_extended && col == n - 1 {
                break;
            }
            let mut pivot = None;
            for row in rank..m {
                if matrix[row][col] != 0 {
                    pivot = Some(row);
                    break;
                }
            }
            if let Some(pivot) = pivot {
                // swap the row
                matrix.swap(pivot, rank);
                let inv = inv_module(matrix[rank][col] as i64, module as i64);
                for x in 0..n {
                    matrix[rank][x] *= inv;
                    matrix[rank][x] %= module;
                }

                // eleminate all pivot's col values(make all values 0)
                for y in 0..m {
                    if y != rank && matrix[y][col] != 0 {
                        let fac = matrix[y][col];
                        for x in 0..n {
                            let sub = (matrix[rank][x] * fac) % module;
                            if matrix[y][x] < sub {
                                matrix[y][x] += module - sub;
                            } else {
                                matrix[y][x] -= sub;
                            }
                        }
                    }
                }
                rank += 1;
            }
        }
        rank
    }

    /// Solve the linear equation
    /// Ax = B
    /// return values are (rank, x, matrix)
    /// If there is no solution, a x is empty.
    pub fn solve(a: &Matrix, b: &[T]) -> (usize, Vec<T>, Matrix) {
        let (m, n) = (a.row, a.col);
        let mut matrix = Matrix::new(m, n + 1);
        for y in 0..m {
            for x in 0..n {
                matrix[y][x] = a[y][x];
            }
            matrix[y][n] = b[y];
        }
        let rank = eliminate(&mut matrix, true);
        for row in rank..m {
            if matrix[row][n] != 0 {
                return (rank, vec![], matrix);
            }
        }
        let mut x = vec![0; n];
        for i in 0..rank {
            x[i] = matrix[i][n];
        }
        (rank, x, matrix)
    }

    /// All most same as `solve()`
    /// module must be prime
    pub fn solve_mod(a: &Matrix, b: &[T], module: T) -> (usize, Vec<T>, Matrix) {
        let (m, n) = (a.row, a.col);
        let mut matrix = Matrix::new(m, n + 1);
        for y in 0..m {
            for x in 0..n {
                matrix[y][x] = (a[y][x] % module + module) % module;
            }
            matrix[y][n] = (b[y] % module + module) % module;
        }
        let rank = eliminate_mod(&mut matrix, true, module);
        for row in rank..m {
            if matrix[row][n] != 0 {
                return (rank, vec![], matrix);
            }
        }
        let mut x = vec![0; n];
        for i in 0..rank {
            x[i] = matrix[i][n];
        }
        (rank, x, matrix)
    }
}
