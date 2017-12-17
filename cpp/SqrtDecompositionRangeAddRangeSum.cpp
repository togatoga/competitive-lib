//@verified http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=2649237#1
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
                eval(k);
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
                eval(k);
                for (int i = max(s, l); i < min(t, r); ++i)
                {
                    sum += data[i];
                }
            }
        }
        return sum;
    }
    void update(int i, T val)
    {
        int k = i / sqrtN;
        eval(k);
        bucket_sum[k] += val - data[i];
        data[i] = val;
    }
    void eval(int k)
    {
        if (bucket_add[k] == 0)
        {
            return;
        }
        for (int i = sqrtN * k; i < sqrtN * (k + 1); i++)
        {
            data[i] += bucket_add[k];
            bucket_sum[k] += bucket_add[k];
        }
        bucket_add[k] = 0;
    }
    T get(int i)
    {
        int k = i / sqrtN;
        return data[i] + bucket_add[k];
    }
};
//end