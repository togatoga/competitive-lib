//Mod Inverse
//Mod must be prime number(ex 10e9 + 7)
template <typename T>
class ModUtil {
public:
  ModUtil(int N, T mod)
      : N(N), mod(mod), inv(N + 1), fact(N + 1), inv_fact(N + 1) {
    calc();
  }
  //
  T get_inv(int n) { return inv[n]; }
  T get_fact(int n) { return fact[n]; }
  T get_inv_fact(int n) { return inv_fact[n]; }
  T get_P(int n, int k) {
    if (n < k)
      return 0;
    return fact[n] * inv_fact[n - k] % mod;
  }
  T get_C(int n, int k) {
    if (n < k)
      return 0;
    return get_P(n, k) * inv_fact[k] % mod;
  }

private:
  vector<T> inv, fact, inv_fact;
  int N;
  T mod;
  // O(N)
  void calc() {
    // cout << N << " " << mod << endl;
    inv[1] = 1;
    for (int i = 2; i <= N; i++) {
      inv[i] = inv[mod % i] * (mod - mod / i) % mod;
    }
    fact[0] = 1;
    for (int i = 1; i <= N; i++) {
      fact[i] = i * fact[i - 1] % mod;
    }
    inv_fact[0] = 1;
    for (int i = 1; i <= N; i++) {
      inv_fact[i] = inv[i] * inv_fact[i - 1] % mod;
      // cout << i << " " << inv_fact[i] << endl;
    }
  }
};
