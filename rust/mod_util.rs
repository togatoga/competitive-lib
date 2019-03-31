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
            inv_fact[i] = mod_pow(fact[i], module - 2, module);
        }
        ModUtil {
            fact: fact,
            inv_fact: inv_fact,
            module: module,
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
        mod_pow(x, self.module - 2, self.module)
    }
    pub fn inv_fact(&self, x: usize) -> usize {
        self.inv(self.fact[x])
    }
}

fn mod_pow(x: usize, n: usize, module: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut res: usize = mod_pow(x, n / 2, module);
    res = (res % module * res % module) % module;
    if n % 2 == 1 {
        res *= x;
    }
    res % module
}