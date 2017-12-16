//@verified http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=2649217#1
#include <vector>
using namespace std;

//begin
template <class T>
class SqrtDecompositionRangeAddRangeSum
{
private:
  int N, K;
  int sqrtN;
  vector<T> data;
  vector<T> bucket_sum;
  vector<T> bucket_add;

public:
  SqrtDecompositionRangeAddRangeSum(int n) : N(n), K(0), sqrtN(1)
  {
    while (sqrtN * sqrtN <= N)
    {
      sqrtN++;
    }
    K = (N + sqrtN - 1) / sqrtN;
    data.assign(K * sqrtN, 0);
    bucket_sum.assign(K, 0);
    bucket_add.assign(K, 0);
  }
  // [s, t)
  void add(int s, int t, T x)
  {
    int p = s / sqrtN;
    int q = t / sqrtN;
    for (int k = p; k <= q; ++k)
    {
      int l = k * sqrtN, r = (k + 1) * sqrtN;
      if (r <= s || t <= l)
        continue;
      if (s <= l && r <= t)
      {
        bucket_add[k] += x;
      }
      else
      {
        for (int i = max(s, l); i < min(t, r); ++i)
        {
          data[i] += x;
          bucket_sum[k] += x;
        }
      }
    }
  }
  // [s, t)
  T get_sum(int s, int t)
  {
    int p = s / sqrtN;
    int q = t / sqrtN;
    T sum = 0;
    for (int k = p; k <= q; ++k)
    {
      int l = k * sqrtN, r = (k + 1) * sqrtN;
      if (r <= s || t <= l)
        continue;
      if (s <= l && r <= t)
      {
        sum += bucket_sum[k] + bucket_add[k] * sqrtN;
      }
      else
      {
        for (int i = max(s, l); i < min(t, r); ++i)
        {
          sum += data[i] + bucket_add[k];
        }
      }
    }
    return sum;
  }
  T get_value(int i)
  {
    int k = i / sqrtN;
    return data[i] + bucket_add[k];
  }
};
//end