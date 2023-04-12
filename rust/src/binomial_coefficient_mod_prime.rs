use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// nCk mod p
/// NOTE: p must be prime
/// space: O(p)
/// construction: O(n)
/// query: O(log_p n)
/// verified: https://atcoder.jp/contests/arc117/submissions/24847708
pub mod binomial_coefficient_mod_prime {

    #[derive(Debug, Clone, Default)]
    pub struct BinomialCoefficient {
        /// (x! mod p) O(p)
        fact_mods: Vec<i64>,
        prime_mod: i64,
    }
    /// A return value is gcd(a, b)
    /// ax + by = gcd(a, b)
    /// this function sets x and y to satisfiy above formula.
    fn extended_gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
        if b == 0 {
            *x = 1;
            *y = 0;
            return a;
        }
        let gcd = extended_gcd(b, a % b, y, x);
        *y -= a / b * *x;
        gcd
    }

    fn module(a: i64, m: i64) -> i64 {
        (a % m + m) % m
    }

    /// find a^-1 (a^-1*a == 1 mod m)
    /// a and m must be relative prime but m isn't necessarily prime.
    fn inv_module(a: i64, m: i64) -> i64 {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        extended_gcd(a, m, &mut x, &mut y);
        module(x, m)
    }

    impl BinomialCoefficient {
        pub fn new(prime_mod: i64) -> BinomialCoefficient {
            let mut fact_mods = vec![1i64; prime_mod as usize];
            for x in 1..prime_mod {
                let i = x as usize;
                fact_mods[i] *= x * fact_mods[i - 1] % prime_mod;
            }
            BinomialCoefficient {
                fact_mods,
                prime_mod,
            }
        }

        /// a mod p O(log_p n)
        /// Condition: n! = a p^e
        fn fact(&self, n: i64, e: &mut usize) -> i64 {
            *e = 0;
            if n == 0 {
                return 1;
            }
            let x = n / self.prime_mod;
            let result = self.fact(x, e) % self.prime_mod;
            *e += x as usize;
            let y = (n % self.prime_mod) as usize;
            if x % 2 != 0 {
                result * ((self.prime_mod - self.fact_mods[y]) % self.prime_mod)
            } else {
                result * self.fact_mods[y] % self.prime_mod
            }
        }

        /// Combination nCk mod p
        /// O(log_p n)
        pub fn combination(&self, n: i64, k: i64) -> i64 {
            if n < k {
                return 0;
            }
            let mut e1 = 0;
            let mut e2 = 0;
            let mut e3 = 0;
            let v1 = self.fact(n, &mut e1);
            let v2 = self.fact(k, &mut e2);
            let v3 = self.fact(n - k, &mut e3);

            assert!(e1 >= e2 + e3);
            if e1 != e2 + e3 {
                0
            } else {
                v1 % self.prime_mod * inv_module(v2 * v3, self.prime_mod) % self.prime_mod
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::binomial_coefficient_mod_prime::BinomialCoefficient;

    fn naive_combs(n: i64, m: i64) -> Vec<Vec<i64>> {
        let n = n as usize;
        let mut comb = vec![vec![0; n + 1]; n + 1];
        comb[0][0] = 1;
        for i in 1..=n {
            comb[i][0] = 1i64;
            for j in 1..=n {
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % m;
            }
        }

        comb
    }
    #[test]
    fn test_combination() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let n = 1000;
        for p in primes {
            let c = BinomialCoefficient::new(p);
            let combs = naive_combs(n, p);

            for i in 1..=n {
                for j in 0..=i {
                    assert_eq!(c.combination(i, j), combs[i as usize][j as usize]);
                }
            }
        }
    }
}
