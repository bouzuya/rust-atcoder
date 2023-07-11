use proconio::input;

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
        k: usize,
    };

    let mut edges = vec![vec![]; k];
    for i in 1..k {
        edges[i % k].push(((10 * i) % k, 0));
        edges[i % k].push(((i + 1) % k, 1));
    }

    let inf = 1_u64 << 60;
    let dist = dijkstra(k, inf, &edges, 1);
    println!("{}", dist[0] + 1);
}
