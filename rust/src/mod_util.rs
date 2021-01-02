use cargo_snippet::snippet;
#[snippet]
pub mod mod_util {
    pub struct ModUtil {
        fact: Vec<usize>,
        inv_fact: Vec<usize>,
        module: usize,
    }

    #[allow(dead_code)]
    impl ModUtil {
        pub fn new(size: usize, module: usize) -> ModUtil {
            let mut fact = vec![0; size + 1];
            let mut inv_fact = vec![0; size + 1];
            fact[0] = 1;
            for i in 1..size + 1 {
                fact[i] = i * fact[i - 1] % module;
            }
            inv_fact[0] = 1;
            for i in 1..size + 1 {
                inv_fact[i] = ModUtil::mod_pow(fact[i], module - 2, module);
            }
            ModUtil {
                fact,
                inv_fact,
                module,
            }
        }

        pub fn permutation(&self, n: usize, k: usize) -> usize {
            assert!(n >= k);
            self.fact[n] % self.module * self.inv_fact(n - k) % self.module
        }
        pub fn combination(&self, n: usize, k: usize) -> usize {
            assert!(n >= k);
            self.fact[n] * self.inv_fact[k] % self.module * self.inv_fact[n - k] % self.module
        }

        pub fn fact(&self, x: usize) -> usize {
            self.fact[x]
        }
        pub fn inv(&self, x: usize) -> usize {
            ModUtil::mod_pow(x, self.module - 2, self.module)
        }
        pub fn inv_fact(&self, x: usize) -> usize {
            self.inv_fact[x]
        }

        fn mod_pow(x: usize, n: usize, module: usize) -> usize {
            if n == 0 {
                return 1;
            }
            let mut res: usize = ModUtil::mod_pow(x, n >> 1, module) % module;
            res = (res * res) % module;
            if n & 1 == 1 {
                res *= x;
            }
            res % module
        }
    }
}

#[cfg(test)]
mod test {
    use super::mod_util::ModUtil;
    #[test]
    fn test_mod_util() {
        const MOD: usize = 1e9 as usize + 7;
        let mod_util = ModUtil::new(1000, MOD);
        //fact
        assert_eq!(mod_util.fact(5), 120);
        assert_eq!(mod_util.fact(10), 3628800);
        assert_eq!(mod_util.fact(500), 688653593);
        assert_eq!(mod_util.fact(1000), 641419708);
        //combination
        assert_eq!(mod_util.combination(5, 3), 10);
        assert_eq!(mod_util.combination(10, 2), 45);
        assert_eq!(mod_util.combination(1000, 500), 159835829);
        //permutation
        assert_eq!(mod_util.permutation(5, 3), 60);
        assert_eq!(mod_util.permutation(10, 3), 720);
        assert_eq!(mod_util.permutation(1000, 999), 641419708);

        //inv fact
        for i in 1..1000 {
            let one = mod_util.fact(i) * mod_util.inv_fact(i) % MOD;
            assert_eq!(one, 1);
        }
    }
}
