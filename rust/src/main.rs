use cargo_snippet::snippet;
use rust::fastio::fastio;
#[snippet(name = "solver", include = "fastio")]
fn run() {
    #[allow(unused_mut, unused_variables)]
    let mut scn = fastio::Scanner::new(std::io::stdin().lock());
}

#[snippet(include = "macros")]
#[snippet(include = "solver")]
fn main() {
    run()
}
