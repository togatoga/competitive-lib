use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod sum2d {
    use std::ops::{Add, AddAssign, Sub};

    #[derive(Debug, Default, Clone)]
    pub struct Sum2d<T> {
        /// 1-index
        data: Vec<Vec<T>>,
    }

    impl<T: Copy + Clone + Add<Output = T> + AddAssign + Sub<Output = T> + Default> Sum2d<T> {
        /// Makes a new `Sum2d` that has `h` x `w` cells.
        pub fn new(height: usize, width: usize) -> Self {
            assert!(height > 0 && width > 0);
            Self {
                data: vec![vec![T::default(); width + 1]; height + 1],
            }
        }

        /// Adds `v` to a cell (`y`, `x`) .
        /// (`y`, `x`) is 0-index
        pub fn add(&mut self, y: usize, x: usize, v: T) {
            self.data[y + 1][x + 1] += v;
        }
        /// Sets `v` to a cell (`y`, `x`).
        /// (`y`, `x`) is 0-index
        pub fn set(&mut self, y: usize, x: usize, v: T) {
            self.data[y + 1][x + 1] = v;
        }

        /// Builds and calculates cumulative sum for cells.
        pub fn build(&mut self) {
            let height = self.data.len();
            let width = self.data[0].len();
            for y in 1..height {
                for x in 1..width {
                    self.data[y][x] = self.data[y][x] + self.data[y - 1][x] + self.data[y][x - 1]
                        - self.data[y - 1][x - 1];
                }
            }
        }

        /// Calculates and returns an area sum.
        /// An area is `sy <= y <= gy and sx <= x <= gx`.
        /// (`sy`, `sx`) and (`gy`, `gx`) is 0-index.
        pub fn query(&self, sy: usize, sx: usize, gy: usize, gx: usize) -> T {
            assert!(sy <= gy && sx <= gx);
            self.data[gy + 1][gx + 1] + self.data[sy][sx]
                - self.data[gy + 1][sx]
                - self.data[sy][gx + 1]
        }
    }
}
