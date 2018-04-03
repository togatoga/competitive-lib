//@verified http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=2757242#1 
//LowestCommonAncestor
//init -> lca(a, b)
template <typename T>
class LowestCommonAncestor
{
	public:
		static const int MAX_LOG_V = 30;
		static const int NONE_NODE = -1;
		vector<int> parents[MAX_LOG_V]; // follow parents 2^k times node index (ex parents[k][v] start v follow parents 2^k times)
		vector<int> depths;             //the depth from root.
		T &edges;
		void dfs(int pos, int pre, int depth = 0)
		{
			parents[0][pos] = pre;
			depths[pos] = depth;
			for (const auto &nxt : edges[pos])
			{
				if (nxt != pre)
				{
					dfs(nxt, pos, depth + 1);
				}
			}
		}

		void init(int root_idx, int V, const T &edges)
		{
			this->depths.resize(V);
			this->edges = edges;
			for (int i = 0; i < MAX_LOG_V; i++)
			{
				parents[i].resize(V);
			}
			dfs(root_idx, NONE_NODE, 0);
			for (int k = 0; k + 1 < MAX_LOG_V; k++)
			{
				for (int v = 0; v < V; v++)
				{
					if (parents[k][v] == NONE_NODE)
					{
						parents[k + 1][v] = NONE_NODE;
					}
					else
					{
						parents[k + 1][v] = parents[k][parents[k][v]];
					}
				}
			}
		}
		//O(logN)
		int lca(int u, int v)
		{

			if (depths[u] > depths[v])
			{
				swap(u, v);
			}

			//follow parents by same level.
			for (int k = 0; k < MAX_LOG_V; k++)
			{
				int diff = depths[v] - depths[u];
				if ((diff >> k) & 1)
				{
					v = parents[k][v];
					// cerr << "depth = " << depths[v] << endl;
				}
			}
			if (u == v)
			{
				return u;
			}

			//binary search to find lcm(u, v)
			for (int k = MAX_LOG_V - 1; k >= 0; k--)
			{
				if (parents[k][u] != parents[k][v])
				{
					u = parents[k][u];
					v = parents[k][v];
				}
			}
			return parents[0][v];
		}
};

