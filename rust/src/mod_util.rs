use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod mod_util {
    use std::collections::BTreeMap;

    /// Calculates a minimum index `i` that meets a^i = b mod m if exists.
    /// `a` and `m` must be relative prime but `m` isn't necessarily prime.
    /// O(sqrt(m))
    pub fn mod_log(mut a: i64, mut b: i64, m: i64) -> Option<i64> {
        a %= m;
        b %= m;
        let sqrt_m = std::cmp::max(1, (m as f64).sqrt() as i64);

        let a_pow = {
            let mut a_pow = BTreeMap::default();
            let mut x = 1;
            for j in 0..sqrt_m {
                a_pow.entry(x).or_insert(j);
                x = (x * a) % m;
            }
            a_pow
        };

        let inv_a = mod_inv(a, m);
        let inv_a_sqrt_m = mod_pow(inv_a, sqrt_m, m);
        let mut x = b;
        for i in 0..sqrt_m {
            if let Some(j) = a_pow.get(&x) {
                let idx = i * sqrt_m + j;
                return Some(idx);
            }
            x = (x * inv_a_sqrt_m) % m;
        }
        None
    }

    /// Calculates x^n mod m.
    pub fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
        if n == 0 {
            return 1;
        }
        let mut res = mod_pow(x, n >> 1, m) % m;
        res = (res * res) % m;
        if n & 1 == 1 {
            res *= x;
        }
        res % m
    }

    /// Calculates a^-1 (a^-1*a == 1 mod m)
    /// `a` and `m` must be relative prime but `m` isn't necessarily prime.
    pub fn mod_inv(a: i64, m: i64) -> i64 {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        extended_gcd(a, m, &mut x, &mut y);
        (x % m + m) % m
    }

    /// A return value is gcd(a, b)
    /// ax + by = gcd(a, b)
    /// this function sets x and y to satisfiy an above formula.
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
}
