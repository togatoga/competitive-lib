// http://codeforces.com/contest/777/submission/25058525
// SegmentTreeMax
template <typename ValueType> 
class SegmentTreeMax {
public:
  SegmentTreeMax() {}
  SegmentTreeMax(int n, ValueType max_value) {
    this->N = 1;
    this->max_value = max_value;
    while (this->N < n)
      this->N *= 2;
    this->data.resize(2 * N - 1, this->max_value);
  }
  // init
  void init(int n, ValueType max_value) {
    this->N = 1;
    this->max_value = max_value;
    while (this->N < n)
      this->N *= 2;
    this->data.resize(2 * N - 1, this->max_value);
  }
  // update k th element
  void update(int k, ValueType val) {
    k += N - 1;
    data[k] = val;
    while (k > 0) {
      k = (k - 1) / 2;
      data[k] = std::max(data[2 * k + 1], data[2 * k + 2]);
    }
  }
  // Max [a, b)
  ValueType max(int a, int b) { return max(a, b, 0, 0, N); }
  ValueType max(int a, int b, int k, int l, int r) {
    if (r <= a or b <= l)
      return max_value;
    if (a <= l and r <= b)
      return data[k];
    int med = (l + r) / 2;
    return std::max(max(a, b, 2 * k + 1, l, med), max(a, b, 2 * k + 2, med, r));
  }

private:
  std::vector<ValueType> data;
  int N;
  ValueType max_value;
};
///////////////////////////////////////
