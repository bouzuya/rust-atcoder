use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let mut edges = vec![vec![]; n];
    for (a, b) in ab.iter().copied() {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut visited = vec![false; n];
    let mut deque = VecDeque::new();
    visited[0] = true;
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            deque.push_back(v);
        }
    }

    let ans = visited.into_iter().all(|b| b);
    println!("{}", if ans { "Yes" } else { "No" });
}
