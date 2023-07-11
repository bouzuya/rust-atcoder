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
    let mut dist = vec![(inf, n); n];
    let mut pq = BinaryHeap::new();
    dist[start] = (0, 0);
    pq.push(Reverse((0, start)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > dist[u].0 {
            continue;
        }
        for (v, w_v) in edges[u].iter().copied() {
            let w = w_u + w_v;
            if w < dist[v].0 {
                dist[v] = (w, u);
                pq.push(Reverse((w, v)));
            }
        }
    }

    let mut path = vec![];
    let mut cur = n - 1;
    while cur != 0 {
        path.push(cur);
        cur = dist[cur].1;
    }
    path.push(cur);

    for u in path.into_iter().rev() {
        println!("{}", u + 1);
    }
}
