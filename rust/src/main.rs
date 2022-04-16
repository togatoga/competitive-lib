use cargo_snippet::snippet;
use rust::fastio::fastio;
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

#[snippet(include = "macros")]
#[snippet(include = "solver")]
fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
