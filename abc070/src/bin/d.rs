use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uvw: &[(usize, usize, u64)]) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &[Vec<(usize, u64)>], s: usize) -> Vec<u64> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for (v, w_v) in e[u].iter().copied() {
            let w = w_u + w_v;
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
        abc: [(Usize1, Usize1, u64); n - 1],
        q: usize,
        k: Usize1,
        xy: [(Usize1, Usize1); q],
    };

    let edges = adjacency_list(n, &abc);
    let inf = 1 << 60;
    let dist = dijkstra(n, inf, &edges, k);
    for (x, y) in xy {
        println!("{}", dist[x] + dist[y]);
    }
}
