use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let edges = adjacency_list(n, &ab);
    let inf = n + 1;
    let mut dist = vec![inf; n];
    let mut deque = VecDeque::new();
    deque.push_back(0);
    dist[0] = 0;
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if dist[v] == inf {
                dist[v] = dist[u] + 1;
                deque.push_back(v);
            }
        }
    }
    if dist[n - 1] == 2 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
