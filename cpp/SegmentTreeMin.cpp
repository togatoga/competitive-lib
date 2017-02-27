//verifiy@http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
//SegmentTreeMin
template<typename ValueType>
class SegmentTreeMin{
public:
  SegmentTreeMin(){}
  SegmentTreeMin(int n, ValueType max_value){
    this->N = 1;
    this->max_value = max_value;
    while (this->N < n) this->N *= 2;
    this->data.resize(2 * N - 1, this->max_value);
  }
  //init
  void init(int n, ValueType max_value){
    this->N = 1;
    this->max_value = max_value;
    while (this->N < n) this->N *= 2;
    this->data.resize(2 * N - 1, this->max_value);
  }
  // update k th element
  void update(int k, ValueType val){
    k += N - 1;
    data[k] = val;
    while (k > 0){
      k = (k - 1) / 2;
      data[k] = std::min(data[2 * k + 1], data[2 * k + 2]);
    }
  }
  //min [a, b)
  ValueType min(int a, int b) {
    return min(a, b, 0, 0, N);
  }
  ValueType min(int a, int b, int k, int l, int r){
    if (r <= a or b <= l) return max_value;
    if (a <= l and r <= b) return data[k];
    int med = (l + r) / 2;
    return std::min(min(a, b, 2 *k + 1, l, med), min(a, b, 2 * k + 2, med, r));
  }
private:
  std::vector<ValueType> data;
  int N;
  ValueType max_value;
};
///////////////////////////////////////
