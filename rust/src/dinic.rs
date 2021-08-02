use cargo_snippet::snippet;
#[snippet]
/// MaxFlow O(N^2m)
/// BipartiteMatching O(mn^(1/2))
/// verified@https://judge.yosupo.jp/problem/bipartitematching
/// verified@https://atcoder.jp/contests/typical90/submissions/24637258
pub mod dinic {
    const INF: i64 = 1i64 << 60;
    use std::collections::VecDeque;
    #[derive(Clone)]
    pub struct Edge {
        to: usize,
        rev: usize,
        cap: i64,
        is_reversed: bool,
    }
    impl Edge {
        pub fn new(to: usize, rev: usize, cap: i64, is_reversed: bool) -> Edge {
            Edge {
                to,
                rev,
                cap,
                is_reversed,
            }
        }

        pub fn to(&self) -> usize {
            self.to
        }
        pub fn is_reversed(&self) -> bool {
            self.is_reversed
        }
        pub fn cap(&self) -> i64 {
            self.cap
        }
    }
    pub struct Dinic {
        pub graph: Vec<Vec<Edge>>,
        level: Vec<i32>,
        graph_indices: Vec<usize>,
        deque: VecDeque<usize>,
    }
    impl Dinic {
        pub fn new(n: usize) -> Dinic {
            Dinic {
                graph: vec![vec![]; n],
                level: vec![-1; n],
                graph_indices: vec![0; n],
                deque: VecDeque::with_capacity(n),
            }
        }
        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            let from_len = self.graph[from].len();
            let to_len = self.graph[to].len();
            self.graph[from].push(Edge {
                to,
                rev: to_len,
                is_reversed: false,
                cap,
            });
            self.graph[to].push(Edge {
                to: from,
                rev: from_len,
                is_reversed: true,
                cap: 0,
            });
        }
        fn bfs(&mut self, s: usize) {
            self.level.iter_mut().for_each(|x| *x = -1);
            self.level[s] = 0;
            assert!(self.deque.is_empty());
            self.deque.push_back(s);
            while let Some(v) = self.deque.pop_front() {
                for edge in self.graph[v].iter() {
                    if edge.cap() > 0 && self.level[edge.to] < 0 {
                        self.level[edge.to] = self.level[v] + 1;
                        self.deque.push_back(edge.to);
                    }
                }
            }
        }
        fn dfs(&mut self, s: usize, t: usize, flow: i64) -> i64 {
            if s == t {
                return flow;
            }
            let l1 = self.level[s];
            while self.graph_indices[s] < self.graph[s].len() {
                let (flow, to, l2) = {
                    let edge = &self.graph[s][self.graph_indices[s]];
                    let flow = std::cmp::min(flow, edge.cap());
                    let to = edge.to;
                    let l2 = self.level[to];
                    (flow, to, l2)
                };
                if flow > 0 && l1 < l2 {
                    let flowed = self.dfs(to, t, flow);
                    if flowed > 0 {
                        let rev = self.graph[s][self.graph_indices[s]].rev;
                        self.graph[s][self.graph_indices[s]].cap -= flowed;
                        self.graph[to][rev].cap += flowed;
                        return flowed;
                    }
                }
                self.graph_indices[s] += 1;
            }
            0
        }
        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let mut flow = 0;
            loop {
                self.bfs(s);
                if self.level[t] < 0 {
                    return flow;
                }
                self.graph_indices.iter_mut().for_each(|x| *x = 0);
                loop {
                    let f = self.dfs(s, t, INF);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
        }
    }

    /// BipartiteMatching O(mn^(1/2))
    pub struct BipartiteMatching {
        dinic: Dinic,
        n1: usize,
        #[allow(dead_code)]
        n2: usize,
        source: usize,
        target: usize,
    }

    impl BipartiteMatching {
        pub fn new(n1: usize, n2: usize) -> BipartiteMatching {
            let source = n1 + n2;
            let target = source + 1;
            let mut dinic = Dinic::new(n1 + n2 + 2);
            for i in 0..n1 {
                dinic.add_edge(source, i, 1);
            }
            for i in 0..n2 {
                dinic.add_edge(n1 + i, target, 1);
            }
            BipartiteMatching {
                dinic,
                n1,
                n2,
                source,
                target,
            }
        }

        pub fn add_edge(&mut self, x1: usize, x2: usize) {
            assert!(x1 < self.n1 && x2 < self.n2);
            assert!(self.n1 + x2 < self.source);
            self.dinic.add_edge(x1, self.n1 + x2, 1);
        }

        pub fn max_match(&mut self) -> Vec<(usize, usize)> {
            let mut results = vec![];
            self.dinic.max_flow(self.source, self.target);
            for i in 0..self.n1 {
                self.dinic.graph[i]
                    .iter()
                    .filter(|e| e.to() < self.source && e.cap() == 0)
                    .for_each(|e| results.push((i, e.to() - self.n1)));
            }
            results
        }
    }
}
