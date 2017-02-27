class XorShift {
  static const int MAX = numeric_limits<int>::max();
  int x = 123456789, y = 362436069, z = 521288629, w = 88675123;

public:
  XorShift() {}
  XorShift(int seed) {
    x ^= seed;
    y ^= x << 10 & seed;
    w ^= y >> 20 | seed;
    z ^= w & x;
  }
  int next() {
    int t = x ^ (x << 11);
    x = y;
    y = z;
    z = w;
    return w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));
  }
  int next(int min_value, int max_value) {
    int range = max_value - min_value;
    return (sample() * range) + min_value;
  }
  double sample() { return next() * 1.0 / MAX; }
};
