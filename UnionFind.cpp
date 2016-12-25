#include <vector>

//UnionFind
class UnionFind {
public:
  UnionFind(int size_) : parent(size_, -1), __size(size_) {}
  void unite(int x, int y) {
    if ((x = find(x)) != (y = find(y))) {
      if (parent[y] < parent[x])
        std::swap(x, y);
      parent[x] += parent[y];
      parent[y] = x;
      __size--;
    }
  }
  bool is_parent(int x) { return find(x) == x; }
  bool is_same(int x, int y) { return find(x) == find(y); }
  int find(int x) { return parent[x] < 0 ? x : parent[x] = find(parent[x]); }
  int size(int x) { return -parent[find(x)]; }
  int size() const { return __size; }

private:
  std::vector<int> parent;
  int __size;
};
//UnionFind
