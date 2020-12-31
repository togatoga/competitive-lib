pub mod fastio {
    use std::collections::VecDeque;
    use std::io::BufWriter;
    use std::io::Write;

    pub struct Writer<W: std::io::Write> {
        writer: std::io::BufWriter<W>,
    }

    impl<W: std::io::Write> Writer<W> {
        pub fn new(write: W) -> Writer<W> {
            Writer {
                writer: BufWriter::new(write),
            }
        }
        pub fn flush(&mut self) {
            self.writer.flush().unwrap();
        }

        pub fn write<S: std::string::ToString>(&mut self, s: S) {
            self.writer.write(s.to_string().as_bytes()).unwrap();
        }
        pub fn writeln<S: std::string::ToString>(&mut self, s: S) {
            self.write(s);
            self.write('\n');
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
