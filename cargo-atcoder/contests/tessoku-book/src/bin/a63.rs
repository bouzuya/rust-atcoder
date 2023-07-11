use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m]
    };

    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let inf = n + 1;
    let mut dist = vec![inf; n];
    let mut deque = VecDeque::new();
    deque.push_back(0);
    dist[0] = 0;
    while let Some(u) = deque.pop_front() {
        let d = dist[u];
        for v in edges[u].iter().copied() {
            if dist[v] == inf {
                dist[v] = d + 1;
                deque.push_back(v);
            }
        }
    }

    for d in dist {
        println!("{}", if d == inf { -1 } else { d as i64 });
    }
}
