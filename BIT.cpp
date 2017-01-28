template <class T> class BIT {
public:
  BIT() {}
  BIT(int N) : N(N) { dat.assign(N, 0); }
  // sum [0,i)
  T sum(int i) {
    int ret = 0;
    for (--i; i >= 0; i = (i & (i + 1)) - 1)
      ret += dat[i];
    return ret;
  }
  // sum [i,j)
  T sum(int i, int j) { return sum(j) - sum(i); }
  // add x to i
  void add(int i, T x) {
    for (; i < N; i |= i + 1)
      dat[i] += x;
  }
  int size() { return N; }

private:
  int N;
  vector<T> dat;
};
