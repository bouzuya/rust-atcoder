use proconio::{input, marker::Usize1};

fn dijkstra(n: usize, inf: usize, e: &[Vec<(usize, usize)>], s: usize) -> Vec<usize> {
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
        m: usize,
        abcd: [(Usize1, Usize1, usize, usize); m],
    };

    let weight = 1_000_000;
    let mut edges = vec![vec![]; n];
    for (a, b, c, d) in abcd.iter().copied() {
        edges[a].push((b, weight * c - d));
        edges[b].push((a, weight * c - d));
    }
    let inf = 1_usize << 60;
    let dist = dijkstra(n, inf, &edges, 0);

    println!(
        "{} {}",
        (dist[n - 1] + weight - 1) / weight,
        weight - dist[n - 1] % weight
    );
}
