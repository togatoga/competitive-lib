use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// A range prime sieve answers primes have the range of [L, R).
/// Space: O((R-L) + sqrt(R))
/// Time: O(sqrt(R)*(loglogsqrt(R) + (R-L)*log(R)))
/// verified@https://atcoder.jp/contests/abc227/submissions/27263064
pub mod range_prime_sieve {
    #[derive(Debug, Clone, Default)]
    pub struct RangePrimeSieve {
        left: i64,
        right: i64,
        boarder: i64,
        small: Vec<i64>,
        /// Prime factors for large number
        large: Vec<Vec<i64>>,
        /// Product of prime factors for large number
        aux: Vec<i64>,
    }
    impl RangePrimeSieve {
        /// Makes a new `RangePrimeSieve` struct that has the range of [L, R).
        pub fn new(left: i64, right: i64) -> RangePrimeSieve {
            assert!(left >= 0 && right > 0);
            assert!(left < right);
            let m = ((right as f64).sqrt() as i64 + 1) as usize;

            let diff = (right - left) as usize;

            let mut small: Vec<i64> = (0..m).map(|i| i as i64).collect();
            let mut large = vec![vec![]; diff];
            let mut aux = vec![1i64; diff];
            for x in 2..right {
                if x * x >= right {
                    break;
                }

                if small[x as usize] < x {
                    continue;
                }
                small[x as usize] = x;
                let mut y = x * x;
                while y < m as i64 {
                    if small[y as usize] == y {
                        small[y as usize] = x;
                    }
                    y += x;
                }
                let mut y = (left + x - 1) / x * x;
                while y < right {
                    let mut z = y;
                    loop {
                        let idx = (y - left) as usize;
                        let tmp = aux[idx];

                        if tmp * tmp > right {
                            break;
                        }
                        large[idx].push(x);
                        aux[idx] *= x;
                        z /= x;
                        if z % x != 0 {
                            break;
                        }
                    }
                    y += x;
                }
            }
            RangePrimeSieve {
                left,
                right,
                boarder: m as i64,
                small,
                large,
                aux,
            }
        }
        /// Returns a boolean whether a number is prime.
        /// Time: O(1)
        pub fn is_prime(&self, x: i64) -> bool {
            if x < self.small.len() as i64 {
                self.small[x as usize] == x
            } else {
                let idx = (x - self.left) as usize;
                self.large[idx].is_empty()
            }
        }
        /// Returns the prime factorization of a number.
        /// Time: log(x)
        pub fn factor(&self, mut x: i64) -> Vec<i64> {
            assert!(self.left <= x && x < self.right);
            let idx = (x - self.left) as usize;
            let mut res = self.large[idx].clone();
            x /= self.aux[idx];
            if x >= self.boarder {
                res.push(x);
                return res;
            }

            while x > 1 {
                res.push(self.small[x as usize]);
                x /= self.small[x as usize];
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_prime_sieve::RangePrimeSieve;

    #[test]
    pub fn test_range_prime_sieve() {
        let left = 1e10 as i64;
        let right = left + 50000;
        let rpf = RangePrimeSieve::new(left, right);
        for x in left..right {
            let mut y = x;
            let mut p = 2;
            let mut factors = vec![];
            while p * p <= y {
                while y % p == 0 {
                    y /= p;
                    factors.push(p);
                }
                p += 1;
            }
            if y != 1 {
                factors.push(y);
            }
            assert_eq!(rpf.factor(x), factors);
            assert_eq!(rpf.is_prime(x), factors.len() == 1);
        }
    }
}
