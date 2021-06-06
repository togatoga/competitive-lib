/// Gaussian elimination
pub mod gaussian_elimination {
    use std::ops::{Index, IndexMut};

    type T = i32;
    #[derive(Debug, Default)]
    pub struct Matrix {
        value: Vec<Vec<T>>,
        row: usize,
        col: usize,
    }
    impl Matrix {
        fn new(row: usize, col: usize) -> Matrix {
            Matrix {
                value: vec![vec![T::default(); col]; row],
                row,
                col,
            }
        }
        fn swap(&mut self, i: usize, j: usize) {
            self.value.swap(i, j);
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
    /// Solve the linear equation
    /// Ax = B
    /// If there is no solution, return None
    pub fn solve(a: &Matrix, b: &[T]) -> Option<Vec<T>> {
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
                return None;
            }
        }
        let mut result = vec![0; n];
        for i in 0..rank {
            result[i] = matrix[i][n];
        }
        Some(result)
    }
}
