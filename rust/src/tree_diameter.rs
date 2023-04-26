use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod tree_diameter {
    #[derive(Debug, Clone, Default)]
    pub struct TreeDiameter {
        n: usize,
        edges: Vec<Vec<usize>>,
    }

    impl TreeDiameter {
        /// Makes `TreeDiameter` with the size of `n`.
        pub fn new(n: usize) -> TreeDiameter {
            TreeDiameter {
                n,
                edges: vec![vec![]; n],
            }
        }

        /// Addes an edge connecting `x` to `y`.
        pub fn add_edge(&mut self, x: usize, y: usize) {
            assert!(x != y);
            assert!(x < self.n && y < self.n);
            self.edges[x].push(y);
            self.edges[y].push(x);
        }

        /// Calculates the diameter of a undirected tree.        
        /// Returns a tuple `(usize, usize, usize)`.
        /// `0` is the diameter of tree.
        /// `1` is two points, a path from `1.0` to `1.1` represents the diameter.        
        pub fn diameter(&self) -> (usize, (usize, usize)) {
            let (_diameter, from) = self.dfs_diameter(0, None);
            let (diameter, to) = self.dfs_diameter(from, None);
            (diameter, (from, to))
        }

        /// Returns a vector of `usize` which represents a path from `from` to `to`.
        pub fn path(&self, from: usize, to: usize) -> Vec<usize> {
            let mut path = vec![];
            path.push(from);
            self.dfs_path(from, None, to, &mut path);
            path
        }

        fn dfs_diameter(&self, pos: usize, pre: Option<usize>) -> (usize, usize) {
            let mut result = (0, pos);
            for &nxt in self.edges[pos].iter().filter(|&&to| Some(to) != pre) {
                let next = self.dfs_diameter(nxt, Some(pos));
                if result.0 < next.0 + 1 {
                    result = (next.0 + 1, next.1);
                }
            }
            result
        }

        fn dfs_path(
            &self,
            pos: usize,
            pre: Option<usize>,
            to: usize,
            path: &mut Vec<usize>,
        ) -> bool {
            if pos == to {
                return true;
            }
            for &nxt in self.edges[pos].iter().filter(|&&to| Some(to) != pre) {
                path.push(nxt);
                if self.dfs_path(nxt, Some(pos), to, path) {
                    return true;
                }
                path.pop();
            }
            false
        }
    }
}