#include <map>
#include <string>
class Trie
{
public:
  Trie()
  {
    this->cnt = 0;
  }

  void insert(const std::string &S, int idx = 0)
  {
    if (idx >= S.size())
    {
      //terminate node
      this->cnt++;
      return;
    }
    if (this->nodes.count(S[idx]) == 0)
    {
      this->nodes[S[idx]] = std::make_shared<Trie>();
    }
    this->nodes[S[idx]]->insert(S, idx + 1);
  }

  bool find_all_match(const std::string &S, int idx = 0)
  {
    if (idx >= S.size())
    {
      return this->cnt > 0;
    }
    if (this->nodes.count(S[idx]) == 0)
    {
      return false;
    }
    return this->nodes[S[idx]]->find_all_match(S, idx + 1);
  }

  bool find_prefix_match(const std::string &S, int idx = 0)
  {
    if (idx >= S.size())
    {
      return true;
    }
    if (this->nodes.count(S[idx]) == 0)
    {
      return false;
    }
    return this->nodes[S[idx]]->find_prefix_match(S, idx + 1);
  }

  int count(const std::string &S, int idx = 0)
  {
    if (idx >= S.size())
    {
      return this->cnt;
    }
    if (this->nodes.count(S[idx]) == 0)
    {
      return 0;
    }
    return this->nodes[S[idx]]->count(S, idx + 1);
  }

private:
  std::map<char, std::shared_ptr<Trie>> nodes;
  int cnt;
};
