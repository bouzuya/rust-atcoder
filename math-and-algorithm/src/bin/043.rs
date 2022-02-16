use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut used = vec![false; n];
    let mut deque = VecDeque::new();
    used[0] = true;
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if used[v] {
                continue;
            }
            used[v] = true;
            deque.push_back(v);
        }
    }
    let ans = !used.contains(&false);
    println!("The graph is {}connected.", if ans { "" } else { "not " });
}
