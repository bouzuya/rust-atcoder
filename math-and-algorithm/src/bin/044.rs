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

    let mut d = vec![-1; n];
    let mut deque = VecDeque::new();
    d[0] = 0;
    deque.push_back((0, 0));
    while let Some((u, d_u)) = deque.pop_front() {
        let d_v = d_u + 1;
        for v in edges[u].iter().copied() {
            if d[v] != -1 {
                continue;
            }
            d[v] = d_v;
            deque.push_back((v, d_v));
        }
    }
    for d_k in d {
        println!("{}", d_k);
    }
}
