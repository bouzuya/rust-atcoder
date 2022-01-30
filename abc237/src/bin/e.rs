use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i64; n],
        uv: [(Usize1, Usize1); m],
    };
    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        let f = |h_x, h_y| -> i64 {
            if h_x >= h_y {
                h_x - h_y
            } else {
                -2 * (h_y - h_x)
            }
        };
        edges[u].push((v, f(h[u], h[v])));
        edges[v].push((u, f(h[v], h[u])));
    }

    let inf = 1 << 60;
    let mut ans = vec![-inf; n];
    let mut pq = BinaryHeap::new();
    pq.push((0, 0));
    ans[0] = 0;
    while let Some((w_u, u)) = pq.pop() {
        if ans[u] != w_u {
            continue;
        }

        for (v, w_v) in edges[u].iter().copied() {
            if w_u + w_v > ans[v] {
                ans[v] = w_u + w_v;
                pq.push((w_u + w_v, v));
            }
        }
    }
    let ans = *ans.iter().max().unwrap();
    println!("{}", ans);
}
