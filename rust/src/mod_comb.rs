use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
///
pub mod mod_comb {
    pub enum PrimeModule {
        Mod1000000007,
        Mod998244353,
    }
    impl PrimeModule {
        fn value(&self) -> i64 {
            match *self {
                PrimeModule::Mod1000000007 => 1000000007,
                PrimeModule::Mod998244353 => 998244353,
            }
        }
    }
    pub struct ModComb {
        fact: Vec<i64>,
        inv_fact: Vec<i64>,
        module: i64,
    }

    #[allow(dead_code)]
    impl ModComb {
        pub fn new(size: usize, module: PrimeModule) -> ModComb {
            let mut fact = vec![0; size + 1];
            let mut inv_fact = vec![0; size + 1];
            fact[0] = 1;
            let module = module.value();
            for i in 1..size + 1 {
                fact[i] = i as i64 * fact[i - 1] % module;
            }
            inv_fact[0] = 1;
            for i in 1..size + 1 {
                inv_fact[i] = ModComb::mod_pow(fact[i], (module - 2) as usize, module);
            }
            ModComb {
                fact,
                inv_fact,
                module,
            }
        }

        pub fn permutation(&self, n: usize, k: usize) -> i64 {
            assert!(n >= k);
            self.fact[n] % self.module * self.inv_fact(n - k) % self.module
        }
        pub fn combination(&self, n: usize, k: usize) -> i64 {
            assert!(n >= k);
            self.fact[n] * self.inv_fact[k] % self.module * self.inv_fact[n - k] % self.module
        }

        pub fn fact(&self, x: usize) -> i64 {
            self.fact[x]
        }
        pub fn inv(&self, x: usize) -> i64 {
            ModComb::mod_pow(x as i64, (self.module - 2) as usize, self.module)
        }
        pub fn inv_fact(&self, x: usize) -> i64 {
            self.inv_fact[x]
        }

        fn mod_pow(x: i64, n: usize, module: i64) -> i64 {
            if n == 0 {
                return 1;
            }
            let mut res = ModComb::mod_pow(x, n >> 1, module) % module;
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
    use super::mod_comb::ModComb;
    #[test]
    fn test_mod_util() {
        let mod_comb = ModComb::new(1000, crate::mod_comb::mod_comb::PrimeModule::Mod1000000007);
        //fact
        assert_eq!(mod_comb.fact(5), 120);
        assert_eq!(mod_comb.fact(10), 3628800);
        assert_eq!(mod_comb.fact(500), 688653593);
        assert_eq!(mod_comb.fact(1000), 641419708);
        //combination
        assert_eq!(mod_comb.combination(5, 3), 10);
        assert_eq!(mod_comb.combination(10, 2), 45);
        assert_eq!(mod_comb.combination(1000, 500), 159835829);
        //permutation
        assert_eq!(mod_comb.permutation(5, 3), 60);
        assert_eq!(mod_comb.permutation(10, 3), 720);
        assert_eq!(mod_comb.permutation(1000, 999), 641419708);

        const MOD: i64 = 1e9 as i64 + 7;
        //inv fact
        for i in 1..1000 {
            let one = mod_comb.fact(i) * mod_comb.inv_fact(i) % MOD;
            assert_eq!(one, 1);
        }
    }
}
