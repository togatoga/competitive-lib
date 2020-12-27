//MaxFlow
//O(F|E|)
//0-index
pub mod fordfulkerson {
    #[derive(Clone)]
    struct Edge {
        from: usize,
        to: usize,
        rev: usize, // An index points an reverse edge (to, from).
        cap: i32,
    }

    impl Edge {
        pub fn new(from: usize, to: usize, rev: usize, cap: i32) -> Edge {
            Edge { from, to, rev, cap }
        }
    }

    struct Graph {
        list: Vec<Vec<Edge>>,
    }

    impl Graph {
        pub fn new(n: usize) -> Graph {
            Graph {
                list: vec![vec![]; n],
            }
        }
        pub fn size(&self) -> usize {
            self.list.len()
        }

        // get a reverse edge of an edge
        pub fn rev_edge_mut(&mut self, e: &Edge) -> &mut Edge {
            &mut self.list[e.to][e.rev]
        }

        // run f into an edge(u, v)
        // the capacity of an edge(u, v) decreases f
        // that of a reverse edge(v, u) increases f
        pub fn run_flow(&mut self, u: usize, idx: usize, f: i32) {
            self.list[u][idx].cap -= f;

            //reverse edgeGraphe::new(from, to, to_rev, cap));
            self.list[to].push(Edge::new(to, from, from_rev, 0));
        }
    }

    struct FordFulkerson {
        seen: Vec<bool>,
        pub graph: Graph,
    }

    impl FordFulkerson {
        // Calculate the maximum flow between s and t(s-t)
        fn max_flow(&mut self, s: usize, t: usize) -> i32 {
            let mut result = 0;
            loop {
                self.seen.iter_mut().for_each(|x| *x = false);
                let flow = self.dfs(s, t, 1 << 30);
                if let Some(flow) = flow {
                    result += flow;
                } else {
                    break;
                }
            }
            result
        }
        fn dfs(&mut self, v: usize, t: usize, f: i32) -> Option<i32> {
            if v == t {
                return Some(f);
            }
            self.seen[v] = true;
            let m: usize = self.graph.list[v].len();
            for i in 0..m {
                let e = g.list[v][i].clone();
                if e.cap == 0 {
                    continue;
                }
                let flow = self.dfs(g, e.to, t, std::cmp::min(f, e.cap));
                if let Some(flow) = flow {
                    self.graph.run_flow(e.from, i, flow);
                    return Some(flow);
                }
            }
            None
        }
    }
}
