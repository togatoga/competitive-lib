use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
//Minimum cost Flow
//O(FElogV)
//verified@http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_B
pub mod primal_dual {
    pub type Cost = i64;
    pub type Flow = i64;
    const INF: Cost = 1i64 << 50;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    #[derive(Clone)]
    pub struct Edge {
        to: usize,
        cap: Flow,
        cost: Cost,
        rev: usize,
        is_reversed: bool,
    }

    impl Edge {
        pub fn new(to: usize, cap: Flow, cost: Cost, rev: usize, is_reversed: bool) -> Edge {
            Edge {
                to,
                cap,
                cost,
                rev,
                is_reversed,
            }
        }
    }
    pub struct PrimalDual {
        graph: Vec<Vec<Edge>>,
        potential: Vec<Cost>,
        min_cost: Vec<Cost>,
        pre_v: Vec<Option<usize>>,
        pre_e: Vec<Option<usize>>,
    }

    impl PrimalDual {
        pub fn new(n: usize) -> PrimalDual {
            PrimalDual {
                graph: vec![vec![]; n],
                potential: vec![0; n],
                min_cost: vec![INF; n],
                pre_v: vec![None; n],
                pre_e: vec![None; n],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: Flow, cost: Cost) {
            let to_rev = self.graph[to].len();
            let from_rev = self.graph[from].len();
            self.graph[from].push(Edge::new(to, cap, cost, to_rev, false));
            self.graph[to].push(Edge::new(from, 0, -cost, from_rev, true));
        }

        pub fn min_cost_flow(&mut self, src: usize, target: usize, mut f: Flow) -> Option<Cost> {
            let n = self.graph.len();
            self.potential.iter_mut().for_each(|x| *x = 0);
            self.pre_v.iter_mut().for_each(|x| *x = None);
            self.pre_e.iter_mut().for_each(|x| *x = None);
            let mut que = BinaryHeap::new();
            let mut result = 0;
            while f > 0 {
                self.min_cost.iter_mut().for_each(|x| *x = INF);
                self.min_cost[src] = 0;

                que.push(Reverse((0, src)));
                while let Some(p) = que.pop() {
                    let (cost, pos) = p.0;
                    if self.min_cost[pos] < cost {
                        continue;
                    }
                    for (i, e) in self.graph[pos].iter().enumerate() {
                        let next_cost = self.min_cost[pos] + e.cost + self.potential[pos]
                            - self.potential[e.to];
                        if e.cap > 0 && self.min_cost[e.to] > next_cost {
                            self.min_cost[e.to] = next_cost;
                            self.pre_v[e.to] = Some(pos);
                            self.pre_e[e.to] = Some(i);
                            que.push(Reverse((next_cost, e.to)));
                        }
                    }
                }
                if self.min_cost[target] == INF {
                    return None;
                }
                (0..n).for_each(|i| self.potential[i] += self.min_cost[i]);
                let mut add_flow = f;
                {
                    let mut v = target;
                    while v != src {
                        let pre_v = self.pre_v[v].unwrap();
                        let pre_e = self.pre_e[v].unwrap();
                        add_flow = std::cmp::min(add_flow, self.graph[pre_v][pre_e].cap);
                        v = pre_v;
                    }
                    f -= add_flow;
                }
                result += add_flow * self.potential[target];
                {
                    let mut v = target;
                    while v != src {
                        let pre_v = self.pre_v[v].unwrap();
                        let pre_e = self.pre_e[v].unwrap();
                        self.graph[pre_v][pre_e].cap -= add_flow;
                        let rev = self.graph[pre_v][pre_e].rev;
                        self.graph[v][rev].cap += add_flow;
                        v = pre_v;
                    }
                }
            }
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use primal_dual::Flow;

    use super::primal_dual;

    #[test]
    fn test_min_cost_flow() {
        let n: usize = 4;
        let f: Flow = 2;
        let mut min_cost_flow = primal_dual::PrimalDual::new(n);
        min_cost_flow.add_edge(0, 1, 2, 1);
        min_cost_flow.add_edge(0, 2, 1, 2);
        min_cost_flow.add_edge(1, 2, 1, 1);
        min_cost_flow.add_edge(1, 3, 1, 3);
        min_cost_flow.add_edge(2, 3, 2, 1);
        let res = min_cost_flow.min_cost_flow(0, n - 1, f);
        assert_eq!(res, Some(6));
    }
}
