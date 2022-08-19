use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i64; n],
        uv: [(Usize1, Usize1); m],
    };

    let mut edges = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        edges[u].push((v, (h[u] - h[v]).abs()));
        edges[v].push((u, (h[u] - h[v]).abs()));
    }

    let inf = 1_i64 << 60;
    let mut d = vec![inf; n];
    d[0] = 0_i64;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), 0));
    while let Some((Reverse(w), u)) = pq.pop() {
        if d[u] != w {
            continue;
        }

        for (v, v_w) in edges[u].iter().copied() {
            let next_w = w + v_w;
            if next_w < d[v] {
                d[v] = next_w;
                pq.push((Reverse(next_w), v));
            }
        }
    }
    let ans = d
        .iter()
        .enumerate()
        .map(|(i, d_i)| -d_i - 3 * (h[i] - h[0]))
        .max()
        .unwrap()
        / 2;
    println!("{}", ans);
}
