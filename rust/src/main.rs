use cargo_snippet::snippet;
use rust::fastio::fastio;

#[allow(dead_code)]
#[derive(PartialEq)]
pub struct NonNan(pub f64);

impl PartialOrd for NonNan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Eq for NonNan {}
impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[snippet(name = "solver", include = "fastio")]
#[derive(Default)]
/// NOTE
/// declare variables to reduce the number of parameters for dp and dfs etc.
pub struct Solver {}
#[snippet(name = "solver")]
impl Solver {
    pub fn solve(&mut self) {
        let stdin = std::io::stdin();
        #[allow(unused_mut, unused_variables)]
        let mut scn = fastio::Scanner::new(stdin.lock());
    }
}

#[snippet(prefix = "#[allow(unused_imports)]\nuse crate::gap_traits::*;")]
#[snippet(include = "macros")]
#[snippet(include = "utils")]
#[snippet(include = "solver")]
#[snippet(include = "gap_traits")]
fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
