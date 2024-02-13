use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abx: [(usize, usize, Usize1); n - 1],
    };

    let mut edges = vec![vec![]; n];
    for (i, (a, b, x)) in abx.into_iter().enumerate() {
        edges[i].push((i + 1, a));
        edges[i].push((x, b));
    }

    let inf = 1_usize << 60;
    let mut dist = vec![inf; n];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), 0));
    dist[0] = 0_usize;
    while let Some((Reverse(d), u)) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        for (v, c) in edges[u].iter().copied() {
            if dist[v] > dist[u] + c {
                dist[v] = dist[u] + c;
                pq.push((Reverse(dist[v]), v));
            }
        }
    }

    let ans = dist[n - 1];
    println!("{}", ans);
}
