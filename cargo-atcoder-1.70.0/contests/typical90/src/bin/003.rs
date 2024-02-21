use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn f(edges: &[Vec<usize>], start: usize) -> Vec<usize> {
    let inf = 1_usize << 60;
    let mut dist = vec![inf; edges.len()];
    let mut deque = VecDeque::new();
    dist[start] = 0_usize;
    deque.push_back(start);
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            deque.push_back(v);
        }
    }
    dist
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let dist = f(&edges, 0);

    let mut max = (0, 0);
    for (i, d) in dist.iter().copied().enumerate() {
        if d > max.1 {
            max = (i, d);
        }
    }

    let dist = f(&edges, max.0);
    let max = dist.iter().max().unwrap();

    let ans = max + 1;
    println!("{}", ans);
}
