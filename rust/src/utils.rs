use cargo_snippet::snippet;
#[snippet]
pub mod utils {
    const DYX: [(isize, isize); 8] = [
        (0, 1),   //right
        (1, 0),   //down
        (0, -1),  //left
        (-1, 0),  //top
        (1, 1),   //down right
        (-1, 1),  //top right
        (1, -1),  //down left
        (-1, -1), //top left
    ];

    pub fn try_adj(y: usize, x: usize, dir: usize, h: usize, w: usize) -> Option<(usize, usize)> {
        let ny = y as isize + DYX[dir].0;
        let nx = x as isize + DYX[dir].1;
        if ny >= 0 && nx >= 0 {
            let ny = ny as usize;
            let nx = nx as usize;
            if ny < h && nx < w {
                Some((ny, nx))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    #[derive(PartialEq, PartialOrd)]
    struct NonNan(pub f64);
    impl Eq for NonNan {}
    impl Ord for NonNan {
        fn cmp(&self, other: &NonNan) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).unwrap()
        }
    }
}
