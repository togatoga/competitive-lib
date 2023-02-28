use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// verified: https://atcoder.jp/contests/abc291/submissions/39308146
pub mod centroid_decomposition {
    #[derive(Debug, Clone, Default)]
    pub struct CentroidDecomposition {
        graph: Vec<Vec<usize>>,
        /// (centroid, depth from a root centrod)
        /// A node has a maximum logn centroids.
        belong: Vec<Vec<(usize, usize)>>,
    }

    impl CentroidDecomposition {
        /// Returns a `CentroidDecomposition` with the number of node is `n`.
        pub fn new(n: usize) -> Self {
            CentroidDecomposition {
                graph: vec![vec![]; n],
                belong: Vec::new(),
            }
        }

        /// Adds an undirected edge between a and b.
        pub fn add_edge(&mut self, x: usize, y: usize) {
            self.graph[x].push(y);
            self.graph[y].push(x);
        }

        /// Returns a tuple (usize, Vec<Vec<usize>>) that means (centroid, childs).
        /// If a node has a non-empty childs, which mean a child is centroid.
        /// Condition: A given `graph` must be tree.
        /// Time: O(nlogn)
        /// Space: O(n)
        pub fn build(&mut self) -> (usize, Vec<Vec<usize>>) {
            let n = self.graph.len();
            let mut childs = vec![vec![]; n];
            let mut dp = vec![0; n];
            let mut is_centroid = vec![false; n];
            let mut belong = vec![vec![]; n];
            let centroid =
                self.inner_build(0, None, &mut childs, &mut dp, &mut is_centroid, &mut belong);
            self.belong = belong;
            (centroid, childs)
        }

        fn belong_dfs(
            pos: usize,
            pre: Option<usize>,
            centroid: usize,
            graph: &[Vec<usize>],
            is_centroid: &[bool],
            belong: &mut [Vec<(usize, usize)>],
            depth: usize,
        ) {
            belong[pos].push((centroid, depth));
            for &nxt in graph[pos].iter() {
                if Some(nxt) == pre || is_centroid[nxt] {
                    continue;
                }
                CentroidDecomposition::belong_dfs(
                    nxt,
                    Some(pos),
                    centroid,
                    graph,
                    is_centroid,
                    belong,
                    depth + 1,
                );
            }
        }

        fn subtree_size(
            &self,
            pos: usize,
            pre: Option<usize>,
            dp: &mut [usize],
            is_centroid: &mut [bool],
        ) {
            let mut sum = 1;
            for &nxt in self.graph[pos].iter() {
                if Some(nxt) == pre || is_centroid[nxt] {
                    continue;
                }
                self.subtree_size(nxt, Some(pos), dp, is_centroid);
                sum += dp[nxt];
            }
            dp[pos] = sum;
        }
        fn inner_build(
            &self,
            root: usize,
            pre: Option<usize>,
            childs: &mut [Vec<usize>],
            dp: &mut [usize],
            is_centroid: &mut [bool],
            belong: &mut [Vec<(usize, usize)>],
        ) -> usize {
            self.subtree_size(root, pre, dp, is_centroid);
            let centroid = {
                let size = dp[root];
                let find_centroid = |mut pos: usize, mut pre: Option<usize>| -> usize {
                    loop {
                        let mut has_majority = false;
                        for &nxt in self.graph[pos].iter() {
                            if Some(nxt) == pre || is_centroid[nxt] {
                                continue;
                            }
                            if dp[nxt] > size / 2 {
                                pre = Some(pos);
                                pos = nxt;
                                has_majority = true;
                                break;
                            }
                        }
                        if !has_majority {
                            return pos;
                        }
                    }
                };
                find_centroid(root, pre)
            };
            if let Some(pre) = pre {
                childs[pre].push(centroid);
            }
            is_centroid[centroid] = true;
            CentroidDecomposition::belong_dfs(
                centroid,
                None,
                centroid,
                &self.graph,
                is_centroid,
                belong,
                0,
            );
            for &nxt in self.graph[centroid].iter() {
                if !is_centroid[nxt] {
                    self.inner_build(nxt, Some(centroid), childs, dp, is_centroid, belong);
                }
            }
            centroid
        }
    }
}
