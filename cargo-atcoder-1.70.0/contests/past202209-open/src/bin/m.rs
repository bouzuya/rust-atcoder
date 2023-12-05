use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        blr: [(usize, usize, usize); m],
    };

    let mut edges = vec![vec![]; n + 1];
    for (i, a_i) in a.iter().copied().enumerate() {
        edges[i].push((i + 1, a_i));
    }
    for (b, l, r) in blr {
        edges[l - 1].push((r, b));
    }
    for i in 0..n {
        edges[i + 1].push((i, 0));
    }

    let inf = 1_usize << 60;
    let mut dist = vec![inf; n + 1];
    let mut pq = BinaryHeap::new();
    dist[0] = 0;
    pq.push((Reverse(0), 0));
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d {
            continue;
        }
        for (v, w) in edges[u].iter().copied() {
            if dist[v] <= d + w {
                continue;
            }
            dist[v] = d + w;
            pq.push((Reverse(d + w), v));
        }
    }

    let ans = dist[n];
    println!("{}", ans);
}
