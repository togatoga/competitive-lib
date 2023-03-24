use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// Lowest common ancestor
pub mod lca {
    pub struct Lca {
        pub parents: Vec<Vec<Option<usize>>>, //parents[k][u]: A parent has 2^k distance from u node
        pub dists: Vec<usize>,                //distance from the root
    }

    impl Lca {
        /// Create a new `Lca` struct
        pub fn new(root: usize, n: usize, edges: &[Vec<usize>]) -> Lca {
            let mut k = 1;
            while (1 << k) < n {
                k *= 2;
            }
            let parents = vec![vec![None; n]; k];
            let dists = vec![0; n];

            let mut lca = Lca { parents, dists };
            lca.dfs(root, None, 0, edges);

            for k in 0..k - 1 {
                for v in 0..n {
                    if let Some(parent) = lca.parents[k][v] {
                        lca.parents[k + 1][v] = lca.parents[k][parent];
                    }
                }
            }
            lca
        }

        fn dfs(&mut self, pos: usize, parent: Option<usize>, dist: usize, edges: &[Vec<usize>]) {
            self.parents[0][pos] = parent;
            self.dists[pos] = dist;

            for &nxt in edges[pos].iter() {
                if Some(nxt) != parent {
                    self.dfs(nxt, Some(pos), dist + 1, edges);
                }
            }
        }

        /// Get the distance from `u` to `v`
        pub fn distance(&self, u: usize, v: usize) -> usize {
            self.dists[u] + self.dists[v] - 2 * self.dists[self.query(u, v)]
        }

        /// Returns a boolean whether `p` is on the path from `u` to `v`
        pub fn is_on_path(&self, u: usize, v: usize, p: usize) -> bool {
            self.distance(u, p) + self.distance(p, v) == self.distance(u, v)
        }

        /// Returns an d-ancestor of `u` if it exists.
        pub fn ancestor(&self, mut u: usize, d: usize) -> Option<usize> {
            let k = self.parents.len();
            for i in (0..k).rev() {
                if d >> i & 1 == 1 {
                    if let Some(parent) = self.parents[i][u] {
                        u = parent;
                    } else {
                        return None;
                    }
                }
            }
            return Some(u);
        }
        /// Get the lowest common ancestor of `u` and `v` LCA(u, v)
        /// log(n)
        pub fn query(&self, u: usize, v: usize) -> usize {
            let (mut u, mut v) = if self.dists[u] < self.dists[v] {
                (v, u)
            } else {
                (u, v)
            };
            let k = self.parents.len();

            for i in 0..k {
                if ((self.dists[u] - self.dists[v]) >> i & 1) > 0 {
                    u = self.parents[i][u].unwrap();
                }
            }
            if u == v {
                return u;
            }

            for i in (0..k).rev() {
                if self.parents[i][u] != self.parents[i][v] {
                    u = self.parents[i][u].unwrap();
                    v = self.parents[i][v].unwrap();
                }
            }
            self.parents[0][u].unwrap()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::lca::Lca;
    #[test]
    fn test_lca() {
        let n: usize = 9;
        let mut edges = vec![vec![]; n];
        edges[0].push(1);
        edges[1].push(2);
        edges[1].push(3);
        edges[0].push(4);
        edges[4].push(5);
        edges[4].push(6);
        edges[6].push(7);
        edges[7].push(8);
        let lca = Lca::new(0, n, &edges);
        assert_eq!(lca.query(2, 8), 0);
        assert_eq!(lca.query(2, 3), 1);
        assert_eq!(lca.query(7, 6), 6);
        assert_eq!(lca.query(0, 1), 0);
        assert_eq!(lca.query(5, 8), 4);
        assert_eq!(lca.query(1, 1), 1);
    }
}
