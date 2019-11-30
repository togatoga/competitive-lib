pub mod rolling_hash {
    const MASK_SIZE: usize = 2;
    static MOD: [u64; MASK_SIZE] = [999999937u64, 1000000007u64];
    const BASE: [u64; MASK_SIZE] = [9973, 10007];
    pub struct RollingHash {
        hash: Vec<Vec<u64>>,
        pow: Vec<Vec<u64>>,
    }

    impl RollingHash {
        pub fn new(s: &[u64]) -> RollingHash {
            let n = s.len();
            let mut hash: Vec<Vec<u64>> = vec![vec![0u64; n + 1]; MASK_SIZE];
            let mut pow: Vec<Vec<u64>> = vec![vec![0u64; n + 1]; MASK_SIZE];
            for i in 0..MASK_SIZE {
                pow[i][0] = 1;
                for j in 0..n {
                    pow[i][j + 1] = pow[i][j] * BASE[i] % MOD[i];
                    hash[i][j + 1] = ((hash[i][j] + s[j]) * BASE[i]) % MOD[i];
                }
            }
            RollingHash {
                hash: hash,
                pow: pow,
            }
        }

        //[l, r)
        pub fn hash1(&self, l: usize, r: usize) -> u64 {
            (self.hash[0][r] + MOD[0] - self.hash[0][l] * self.pow[0][r - l] % MOD[0]) % MOD[0]
        }
        //[l, r)
        pub fn hash2(&self, l: usize, r: usize) -> u64 {
            (self.hash[1][r] + MOD[1] - self.hash[1][l] * self.pow[1][r - l] % MOD[1]) % MOD[1]
        }
        //[l, r)
        pub fn hash(&self, l: usize, r: usize) -> (u64, u64) {
            (self.hash1(l, r), self.hash2(l, r))
        }
        pub fn equal(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
            self.hash1(l1, r1) == self.hash1(l2, r2) && self.hash2(l1, r1) == self.hash2(l2, r2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rolling_hash;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_rolling_hash() {
        let seq: Vec<u64> = "abcabc".chars().map(|c| c as u64).collect();
        let rh = rolling_hash::RollingHash::new(&seq);
        assert_eq!(rh.hash(0, 3), rh.hash(3, 6));
        assert_eq!(rh.hash(0, 1), rh.hash(3, 4));
        assert!(rh.equal(0, 3, 3, 6));
        assert_ne!(rh.hash(0, 4), rh.hash(0, 3));

        let seq: Vec<u64> = "xy".chars().map(|c| c as u64).collect();
        assert_ne!(rh.hash(0, 1), rh.hash(1, 2));
        assert_ne!(rh.hash(0, 2), rh.hash(1, 2));
    }
    #[test]
    fn test_random_rolling_hash() {
        let max_length = 50;
        let mut rng = thread_rng();

        for len in 1..max_length + 1 {
            let chars: String = std::iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(len)
                .collect();
            let seq: Vec<u64> = chars.chars().map(|c| c as u64).collect();
            let rh = rolling_hash::RollingHash::new(&seq);
            for i in 0..len {
                for j in i..len {
                    for k in (j + 1)..len {
                        let t = k - j + 1;
                        let equal = chars[i..i + t] == chars[j..j + t];
                        let hash_equal = rh.equal(i, i + t, j, j + t);
                        assert_eq!(equal, hash_equal);
                    }
                }
            }
        }
    }
}
