use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dijkstra(n: usize, inf: usize, edges: &[Vec<usize>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for v in edges[u].iter().copied() {
            let w = w_u + 1;
            if w < d[v] {
                d[v] = w;
                pq.push(Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        k: usize,
        a: [Usize1; k],
        uv: [(Usize1, Usize1); m],
        st: [(Usize1, Usize1); q],
    };

    let inf = 1 << 60;
    let edges = adjacency_list(n, &uv);
    let mut dists = vec![];
    for a_i in a {
        let dist = dijkstra(n, inf, &edges, a_i);
        dists.push(dist);
    }

    for (s, t) in st {
        let mut min = inf;
        for dist in dists.iter() {
            min = min.min(dist[s] + dist[t]);
        }
        println!("{}", min);
    }
}
