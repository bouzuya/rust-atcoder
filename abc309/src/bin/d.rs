use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n_1: usize,
        n_2: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut edges = vec![vec![]; n_1 + n_2];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut deque = VecDeque::new();
    let inf = n_1 + n_2 + 1;
    let mut dist = vec![inf; n_1 + n_2];
    deque.push_back(0);
    dist[0] = 0;
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            deque.push_back(v);
        }
    }

    let mut deque = VecDeque::new();
    deque.push_back(n_1 + n_2 - 1);
    dist[n_1 + n_2 - 1] = 0;
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            deque.push_back(v);
        }
    }

    let ans = dist[0..n_1].iter().max().unwrap() + 1 + dist[n_1..n_1 + n_2].iter().max().unwrap();
    println!("{}", ans);
}
