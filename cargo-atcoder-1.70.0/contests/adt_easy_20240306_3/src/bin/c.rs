use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
    };
    let mut edges = vec![vec![]; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        edges[i + 1].push(p_i);
    }

    let mut depth = vec![0; n];
    let mut deque = VecDeque::new();
    deque.push_back(n - 1);
    depth[n - 1] = 0;
    while let Some(v) = deque.pop_front() {
        for u in edges[v].iter().copied() {
            deque.push_back(u);
            depth[u] = depth[v] + 1;
        }
    }
    let ans = depth[0];
    println!("{}", ans);
}
