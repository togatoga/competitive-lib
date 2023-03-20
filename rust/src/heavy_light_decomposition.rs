use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// Verified: https://atcoder.jp/contests/abc294/submissions/39902418
pub mod heavy_light_decomposition {
    use std::mem::swap;
    #[derive(Debug, Default, Clone)]
    pub struct HeavyLightDecomposition {
        n: usize,
        /// If `x` isn't a leaf node, then calling `build`, `edges[x][0]` is always heavy-edge.
        edges: Vec<Vec<usize>>,
        heavy_array: Vec<usize>,
        /// If `x` and `y` belong to a same heavy component, `heavy_left_indices[x]` and `heavy_left_indices[y]` value(index) are same.
        heavy_left_indices: Vec<usize>,
        /// The lower index is likely to be a lower tree-depth.
        heavy_indices: Vec<usize>,
        parents: Vec<Option<usize>>,
    }

    impl HeavyLightDecomposition {
        /// Returns a new `HeavyLightDecomposition` with the number of node.
        pub fn new(n: usize) -> Self {
            HeavyLightDecomposition {
                n,
                edges: vec![vec![]; n],
                heavy_array: vec![0; n],
                heavy_left_indices: vec![0; n],
                heavy_indices: vec![0; n],
                parents: vec![None; n],
            }
        }

        /// Returns a boolean indicating whether `u` and `v` is same heavy component.
        pub fn is_same_heavy_component(&self, u: usize, v: usize) -> bool {
            self.heavy_left_index(u) == self.heavy_left_index(v)
        }
        /// Returns a left index for a heavy component that `x` belongs.
        pub fn heavy_left_index(&self, x: usize) -> usize {
            self.heavy_left_indices[x]
        }
        /// Returns an index for `heavy_array`.
        pub fn heavy_index(&self, x: usize) -> usize {
            self.heavy_indices[x]
        }

        /// Returns a tuple (usize, usize) array which represents a path from `src` to `dest`.
        /// A returned tuple is an index range `[left, right]` for heavy component.
        pub fn path(&self, src: usize, dest: usize) -> Vec<(usize, usize)> {
            let mut u = src;
            let mut v = dest;
            let mut paths = vec![];
            loop {
                if self.heavy_index(u) > self.heavy_index(v) {
                    swap(&mut u, &mut v);
                }
                if self.is_same_heavy_component(u, v) {
                    if u != v {
                        paths.push((self.heavy_index(u) + 1, self.heavy_index(v)));
                    }
                    break;
                }

                paths.push((self.heavy_left_index(v), self.heavy_index(v)));
                let left_idx = self.heavy_left_index(v);
                let parent = self.parents[self.heavy_array[left_idx]].expect("no parent");
                v = parent;
            }
            paths
        }
        /// Returns a lowest common ancestor for `u` and `v`.
        pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
            loop {
                if self.heavy_index(u) > self.heavy_index(v) {
                    swap(&mut u, &mut v);
                }
                // Same heavy component
                if self.is_same_heavy_component(u, v) {
                    return u;
                }
                let left_idx = self.heavy_left_index(v);
                v = self.parents[self.heavy_array[left_idx]].expect("no parent");
            }
        }
        /// Adds a new edge between `u` and `v`.
        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.edges[u].push(v);
            self.edges[v].push(u);
        }

        /// Builds a heavy-light decomposition.
        pub fn build(&mut self, root: usize) {
            let mut subtree_sizes = vec![0; self.n];
            self.build_heavy_light_edges(root, None, &mut subtree_sizes);

            let mut idx = 0;
            self.heavy_array[idx] = root;
            self.heavy_indices[root] = idx;
            idx += 1;
            self.build_heavy_array(root, None, &mut idx);
        }
        fn build_heavy_array(&mut self, pos: usize, pre: Option<usize>, idx: &mut usize) {
            for i in 0..self.edges[pos].len() {
                let nxt = self.edges[pos][i];
                if Some(nxt) == pre {
                    continue;
                }

                // self.edges[pos][0] is havey-edge.
                self.heavy_array[*idx] = nxt;
                self.parents[nxt] = Some(pos);
                self.heavy_indices[nxt] = *idx;
                *idx += 1;
                if i == 0 {
                    self.heavy_left_indices[nxt] = self.heavy_left_indices[pos];
                } else {
                    self.heavy_left_indices[nxt] = self.heavy_indices[nxt];
                }

                self.build_heavy_array(nxt, Some(pos), idx);
            }
        }
        fn build_heavy_light_edges(
            &mut self,
            pos: usize,
            pre: Option<usize>,
            subtree_sizes: &mut [usize],
        ) {
            if let Some(&first) = self.edges[pos].first() {
                if Some(first) == pre {
                    let n = self.edges[pos].len();
                    self.edges[pos].swap(0, n - 1);
                }
            }
            subtree_sizes[pos] += 1;
            for i in 0..self.edges[pos].len() {
                let nxt = self.edges[pos][i];
                if Some(nxt) == pre {
                    continue;
                }
                self.build_heavy_light_edges(nxt, Some(pos), subtree_sizes);
                subtree_sizes[pos] += subtree_sizes[nxt];
                if subtree_sizes[nxt] > subtree_sizes[self.edges[pos][0]] {
                    // edges[pos][0] must be havey-edge.
                    self.edges[pos].swap(0, i);
                }
            }
        }
    }
}
