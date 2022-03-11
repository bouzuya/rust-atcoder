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
        m: usize,
        abc: [(Usize1, Usize1, u64); m],
    };

    let edges = adjacency_list(n, &abc);
    let inf = 1_u64 << 60;
    let dist = dijkstra(n, inf, &edges, 0);
    let ans = if dist[n - 1] == inf {
        -1_i64
    } else {
        dist[n - 1] as i64
    };
    println!("{}", ans);
}
