pub mod rolling_hash {
    const MASK_SIZE: usize = 2;
    static MOD: [u64; MASK_SIZE] = [999999937u64, 1000000007u64];
    const BASE: u64 =  9973;
    struct RollingHash {
        hash: Vec<Vec<u64>>,
        pow: Vec<Vec<u64>>,
    }

    impl RollingHash {
        pub fn new<T>(s: &[T]) -> RollingHash {
            let n = s.len();
            let mut hash: Vec<Vec<u64>> = vec![vec![0u64; n + 1]; 2];
            let mut pow: Vec<Vec<u64>> = vec![vec![0u64; n + 1]; 2];
            for i in 0..MASK_SIZE {
                hash[i][0] = 1;
                pow[i][0] = 1;
                for j in 0..n {
                    pow[i][j + 1] = pow[i][i] * BASE % MOD[i];
                    hash[i][j + 1] = (hash[i][j] * BASE + s[j] as u64) % MOD[i];
                }
            }

            RollingHash {
                hash: hash,
                pow: pow,
            }
        }

    }
}
