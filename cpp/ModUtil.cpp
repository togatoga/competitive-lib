//@verified http://codeforces.com/contest/559/submission/33876738
template <typename T>
class ModUtil
{
  public:
    ModUtil(int N, T mod) : N(N), mod(mod), fact(N + 1), inv_fact(N + 1) { calc(); }

    T get_inv(int x) { return modpow(x, mod - 2); }
    T get_fact(int n) { return fact[n]; }
    T get_inv_fact(int n) { return get_inv(fact[n]); }
    T get_P(int n, int k)
    {
        if (n < k)
            return 0;
        return fact[n] % mod * get_inv_fact(n - k) % mod;
    }
    T get_C(int n, int k)
    {
        if (n < k)
            return 0;
        return fact[n] * inv_fact[k] % mod * inv_fact[n - k] % mod;
    }
    T modpow(int a, int n)
    {
        if (n == 0)
            return 1;
        T res = modpow(a, n / 2);
        res *= res;
        res %= mod;
        if (n % 2)
            res *= a;
        return res % mod;
    }

  private:
    vector<T> fact;
    vector<T> inv_fact;
    int N;
    T mod;
    // O(N)
    void calc()
    {
        fact[0] = 1;
        for (int i = 1; i <= N; i++)
        {
            fact[i] = i * fact[i - 1] % mod;
        }
        inv_fact[0] = 1;
        for (int i = 1; i <= N; i++)
        {
            inv_fact[i] = modpow(fact[i], mod - 2);
        }
    }
};