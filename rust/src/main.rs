#[derive(Default)]
/// NOTE
/// declare variables to reduce the number of parameters for dp and dfs etc.
struct Solver {}

impl Solver {
    fn solve(&mut self) {
        // let stdout = std::io::stdout();
        // let mut wrt = fastio::Writer::new(stdout.lock());
        let stdin = std::io::stdin();
        let mut scn = fastio::Scanner::new(stdin.lock());
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| {
            let mut solver = Solver::default();
            solver.solve()
        })
        .unwrap()
        .join()
        .unwrap();
}
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

pub mod fastio {
    use std::collections::VecDeque;
    use std::io::BufWriter;
    use std::io::Write;

    pub struct Writer<W: std::io::Write> {
        writer: std::io::BufWriter<W>,
        buffer: VecDeque<Vec<u8>>,
    }

    impl<W: std::io::Write> Writer<W> {
        pub fn new(write: W) -> Writer<W> {
            Writer {
                writer: BufWriter::new(write),
                buffer: VecDeque::new(),
            }
        }
        pub fn flush(&mut self) {
            while let Some(p) = self.buffer.pop_front() {
                self.writer.write(&p).unwrap();
            }
            self.writer.flush().unwrap();
        }

        pub fn write<S: std::string::ToString>(&mut self, s: S) {
            self.buffer.push_back(s.to_string().as_bytes().to_vec());
        }
        pub fn writeln<S: std::string::ToString>(&mut self, s: S) {
            self.write(s.to_string() + "\n");
        }
    }

    pub struct Scanner<R> {
        stdin: R,
        buffer: VecDeque<String>,
    }

    impl<R: std::io::BufRead> Scanner<R> {
        pub fn new(s: R) -> Scanner<R> {
            Scanner {
                stdin: s,
                buffer: VecDeque::new(),
            }
        }
        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            while self.buffer.is_empty() {
                let line = self.read_line();
                for w in line.split_whitespace() {
                    self.buffer.push_back(String::from(w));
                }
            }
            self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
        }
        pub fn read_line(&mut self) -> String {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            line
        }
        pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.read()).collect()
        }

        pub fn chars(&mut self) -> Vec<char> {
            self.read::<String>().chars().collect()
        }
    }
}

// utility macros
#[macro_export]
#[allow(unused_macros)]
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        std::cmp::max($x, max!($($y),+))
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        std:::cmp::min($x, min!($($y),+))
    }
}
