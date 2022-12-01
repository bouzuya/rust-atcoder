use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m],
    };

    let mut edges = vec![vec![]; n];
    for (u, v, w) in abc {
        edges[u].push((v, w));
        edges[v].push((u, w));
    }

    let inf = 1_u64 << 60;
    let start = 0;
    let mut dist = vec![inf; n];
    let mut pq = BinaryHeap::new();
    dist[start] = 0;
    pq.push(Reverse((0, start)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > dist[u] {
            continue;
        }
        for (v, w_v) in edges[u].iter().copied() {
            let w = w_u + w_v;
            if w < dist[v] {
                dist[v] = w;
                pq.push(Reverse((w, v)));
            }
        }
    }

    for dist_k in dist {
        println!("{}", if dist_k == inf { -1 } else { dist_k as i64 });
    }
}
