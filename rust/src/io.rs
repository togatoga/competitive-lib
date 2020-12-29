pub mod io {
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
        pub stdin: R,
    }

    impl<R: std::io::Read> Scanner<R> {
        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            use std::io::Read;
            let buf = self
                .stdin
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
                .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
                .collect::<Vec<_>>();
            unsafe { std::str::from_utf8_unchecked(&buf) }
                .parse()
                .ok()
                .expect("Parse error.")
        }
        pub fn read_line(&mut self) -> String {
            use std::io::Read;
            let buf = self
                .stdin
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b'\n' || b == b'\r')
                .take_while(|&b| b != b'\n' && b != b'\r')
                .collect::<Vec<_>>();
            unsafe { std::str::from_utf8_unchecked(&buf) }
                .parse()
                .ok()
                .expect("Parse error.")
        }
        pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.read()).collect()
        }

        pub fn chars(&mut self) -> Vec<char> {
            self.read::<String>().chars().collect()
        }
    }
}
