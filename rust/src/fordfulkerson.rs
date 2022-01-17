use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// MaxFlow
/// O(F|E|)
/// 0-index
/// verified@http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5091370#1
pub mod fordfulkerson {
    #[derive(Clone)]
    pub struct Edge {
        from: usize,
        to: usize,
        rev: usize, // An index points an reverse edge (to, from).
        cap: i64,
        is_reversed: bool,
    }

    impl Edge {
        pub fn new(from: usize, to: usize, rev: usize, cap: i64, is_reversed: bool) -> Edge {
            Edge {
                from,
                to,
                rev,
                cap,
                is_reversed,
            }
        }
        pub fn from_to(&self) -> (usize, usize) {
            (self.from, self.to)
        }
        pub fn is_reversed(&self) -> bool {
            self.is_reversed
        }
        pub fn cap(&self) -> i64 {
            self.cap
        }
    }

    pub struct Graph {
        pub list: Vec<Vec<Edge>>,
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
        pub fn run_flow(&mut self, u: usize, idx: usize, f: i64) {
            self.list[u][idx].cap -= f;

            //reverse edgeGraphe::new(from, to, to_rev, cap));
            let e = &self.list[u][idx].clone();
            self.rev_edge_mut(e).cap += f;
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            let from_rev: usize = self.list[from].len();
            let to_rev: usize = self.list[to].len();
            self.list[from].push(Edge::new(from, to, to_rev, cap, false));
            self.list[to].push(Edge::new(to, from, from_rev, 0, true));
        }
    }

    pub struct FordFulkerson {
        seen: Vec<bool>,
        pub graph: Graph,
    }

    impl FordFulkerson {
        pub fn new(n: usize) -> FordFulkerson {
            FordFulkerson {
                seen: vec![false; n],
                graph: Graph::new(n),
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            self.graph.add_edge(from, to, cap);
        }

        // Calculate the maximum flow between s and t(s-t)
        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let mut result = 0;
            loop {
                self.seen.iter_mut().for_each(|x| *x = false);
                let flow = self.dfs(s, t, std::i64::MAX);
                if let Some(flow) = flow {
                    result += flow;
                } else {
                    break;
                }
            }
            result
        }
        fn dfs(&mut self, v: usize, t: usize, f: i64) -> Option<i64> {
            if v == t {
                return Some(f);
            }
            self.seen[v] = true;
            let m: usize = self.graph.list[v].len();
            for i in 0..m {
                let e = self.graph.list[v][i].clone();
                if self.seen[e.to] {
                    continue;
                }
                if e.cap == 0 {
                    continue;
                }
                let flow = self.dfs(e.to, t, std::cmp::min(f, e.cap));
                if let Some(flow) = flow {
                    self.graph.run_flow(e.from, i, flow);
                    return Some(flow);
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fordfulkerson;

    #[test]
    fn test_max_flow() {
        //http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A&lang=jp
        let mut ff = fordfulkerson::FordFulkerson::new(4);
        ff.add_edge(0, 1, 2);
        ff.add_edge(0, 2, 1);
        ff.add_edge(1, 2, 1);
        ff.add_edge(1, 3, 1);
        ff.add_edge(2, 3, 2);

        let max_flow = ff.max_flow(0, 3);
        assert_eq!(max_flow, 3);
    }
}
