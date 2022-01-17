use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// c[i + j] = a[i] * a[j] (1 <= i <= n, 1 <= j <= m)
/// @verified: https://atcoder.jp/contests/atc001/submissions/24732479
pub mod karatsuba {
    fn karatsuba<T>(a: &[T], b: &[T], c: &mut [T], buf: &mut [T])
    where
        T: std::marker::Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + Default,
    {
        let n = a.len();
        if n <= 16 {
            for (i, a) in a.iter().enumerate() {
                for (c, b) in c[i..].iter_mut().zip(b.iter()) {
                    *c = *c + *a * *b;
                }
            }
            return;
        }
        if n & 1 == 1 {
            karatsuba(&a[1..], &b[1..], &mut c[2..], buf);
            let x = a[0];
            let y = b[0];
            c[0] = c[0] + x * y;
            for (c, (a, b)) in c[1..].iter_mut().zip(a[1..].iter().zip(b[1..].iter())) {
                *c = *c + x * *b + *a * y;
            }
            return;
        }
        let m = n / 2;
        karatsuba(&a[..m], &b[..m], &mut c[..n], buf);
        karatsuba(&a[m..], &b[m..], &mut c[n..], buf);
        let (x, y) = buf.split_at_mut(m);
        let (y, z) = y.split_at_mut(m);
        let (z, buf) = z.split_at_mut(n);
        z.iter_mut().for_each(|z| *z = T::default());

        for (x, (p, q)) in x.iter_mut().zip(a.iter().zip(a[m..].iter())) {
            *x = *p + *q;
        }
        for (y, (p, q)) in y.iter_mut().zip(b.iter().zip(b[m..].iter())) {
            *y = *p + *q;
        }
        karatsuba(x, y, z, buf);
        for (z, (p, q)) in z.iter_mut().zip(c[..n].iter().zip(c[n..].iter())) {
            *z = *z - (*p + *q);
        }
        for (c, z) in c[m..].iter_mut().zip(z.iter()) {
            *c = *c + *z;
        }
    }

    pub fn multiply<T>(a: &[T], b: &[T]) -> Vec<T>
    where
        T: std::marker::Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + Default,
    {
        let mut i = 0;
        let mut j = 0;
        let mut ans = vec![T::default(); a.len() + b.len()];
        let mut buf = vec![T::default(); 4 * (a.len() + b.len())];
        let mut c = Vec::with_capacity(a.len() + b.len());
        while i < a.len() && j < b.len() {
            let x = a.len() - i;
            let y = b.len() - j;
            let z = std::cmp::min(x, y);
            c.clear();
            c.resize(2 * z, T::default());
            karatsuba(&a[i..(i + z)], &b[j..(j + z)], &mut c, &mut buf);
            for (ans, c) in ans[(i + j)..].iter_mut().zip(c.iter()) {
                *ans = *ans + *c;
            }
            if x <= y {
                j += x;
            } else {
                i += y;
            }
        }
        ans.truncate(a.len() + b.len() - 1);
        ans
    }
}

#[cfg(test)]
mod test {

    use super::karatsuba::multiply;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_multiply() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 4, 8];

        let mut c = vec![0; a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                c[i + j] += *x * *y;
            }
        }
        let c1 = multiply(&a, &b);
        assert_eq!(c1, c);
    }

    #[test]
    fn test_mod_int_multiply() {
        use crate::mod_int::mod_int;
        type ModInt = mod_int::ModInt<usize, mod_int::Mod1000000007>;
        let n = 10000;
        let mut rng = thread_rng();
        let a = (0..n)
            .map(|_| ModInt::new(0) + rng.gen_range(1000000, 10000000))
            .collect::<Vec<_>>();
        let b = (0..n)
            .map(|_| ModInt::new(0) + rng.gen_range(1000000, 10000000))
            .collect::<Vec<_>>();
        let mut c: Vec<ModInt> = (0..2 * n - 1).map(|_| ModInt::new(0)).collect();
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                c[i + j] += *x * *y;
            }
        }

        assert_eq!(multiply(&a, &b), c);
    }
}
