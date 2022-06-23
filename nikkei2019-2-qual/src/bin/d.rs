use proconio::{input, marker::Usize1};

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
        lrc: [(Usize1, Usize1, u64); m],
    };

    let mut edges = vec![vec![]; n];
    for i in (0..n).skip(1) {
        edges[i].push((i - 1, 0));
    }
    for (l_i, r_i, c_i) in lrc {
        edges[l_i].push((r_i, c_i));
    }

    let inf = 1 << 60;
    let d = dijkstra(n, inf, &edges, 0);
    if d[n - 1] == inf {
        println!("-1");
    } else {
        println!("{}", d[n - 1]);
    }
}
