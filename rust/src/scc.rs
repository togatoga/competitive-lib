use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod scc {
    //Verified: https://atcoder.jp/contests/practice2/submissions/18811349
    //Kosaraju’s Algorithm
    //O(|V| + |E|)
    pub struct SccGraph {
        n: usize,
        graphs: Vec<Vec<usize>>,     // graphs 0 -> 1
        rev_graphs: Vec<Vec<usize>>, // reverse graphs 1 -> 0
        post_order: Vec<usize>,      // the post order of dfs
        scc_indices: Vec<usize>,     // the indices of strongly connected component
        visited: Vec<bool>,
    }

    impl SccGraph {
        pub fn new(n: usize) -> SccGraph {
            let graphs = vec![vec![]; n];
            let rev_graphs = vec![vec![]; n];
            let post_order = vec![];
            let scc_indices = vec![0; n];
            let visited = vec![false; n];

            SccGraph {
                n,
                graphs,
                rev_graphs,
                post_order,
                scc_indices,
                visited,
            }
        }
        //0-index
        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.internal_add_edge(from, to);
        }
        pub fn scc(&mut self) -> Vec<Vec<usize>> {
            self.internal_scc()
        }
        fn internal_add_edge(&mut self, from: usize, to: usize) {
            self.graphs[from].push(to);
            self.rev_graphs[to].push(from);
        }
        fn dfs(&mut self, pos: usize) {
            self.visited[pos] = true;
            let m = self.graphs[pos].len();
            for i in 0..m {
                let nxt = self.graphs[pos][i];
                if !self.visited[nxt] {
                    self.dfs(nxt);
                }
            }
            self.post_order.push(pos);
        }

        fn rev_dfs(&mut self, pos: usize, label: usize) {
            self.visited[pos] = true;
            self.scc_indices[pos] = label;
            let m = self.rev_graphs[pos].len();
            for i in 0..m {
                let nxt = self.rev_graphs[pos][i];
                if !self.visited[nxt] {
                    self.rev_dfs(nxt, label);
                }
            }
        }
        fn internal_scc(&mut self) -> Vec<Vec<usize>> {
            for i in 0..self.n {
                if !self.visited[i] {
                    self.dfs(i);
                }
            }

            self.visited.iter_mut().for_each(|x| *x = false);
            let m = self.post_order.len();
            let mut label = 0;
            for i in (0..m).rev() {
                let pos = self.post_order[i];
                if !self.visited[pos] {
                    self.rev_dfs(pos, label);
                    label += 1;
                }
            }

            let mut groups = vec![vec![]; label];
            for i in 0..self.n {
                groups[self.scc_indices[i]].push(i);
            }
            groups
        }
    }
}

#[cfg(test)]
mod tests {
    use super::scc::SccGraph;

    #[test]
    fn test_scc() {
        let n = 12;

        let mut scc = SccGraph::new(n);
        scc.add_edge(11, 10);
        scc.add_edge(10, 9);
        scc.add_edge(10, 8);
        scc.add_edge(9, 8);
        scc.add_edge(8, 7);
        scc.add_edge(7, 9);
        scc.add_edge(7, 6);
        scc.add_edge(6, 5);
        scc.add_edge(4, 6);
        scc.add_edge(5, 4);
        scc.add_edge(5, 3);
        scc.add_edge(5, 2);
        scc.add_edge(2, 3);
        scc.add_edge(2, 1);
        scc.add_edge(3, 0);
        scc.add_edge(0, 3);

        let groups = scc.scc();
        assert_eq!(groups[0], vec![11]);
        assert_eq!(groups[1], vec![10]);
        assert_eq!(groups[2], vec![7, 8, 9]);
        assert_eq!(groups[3], vec![4, 5, 6]);
        assert_eq!(groups[4], vec![2]);
        assert_eq!(groups[5], vec![1]);
        assert_eq!(groups[6], vec![0, 3]);
    }

    #[test]
    fn solve_alpc_g_sample1() {
        // https://atcoder.jp/contests/practice2/tasks/practice2_g
        let n: usize = 6;
        let edges = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];

        let mut graph = SccGraph::new(n);
        for (u, v) in edges.into_iter() {
            graph.add_edge(u, v);
        }

        let scc = graph.scc();
        assert_eq!(scc, vec![vec![5], vec![1, 4], vec![2], vec![0, 3]]);
    }
}
