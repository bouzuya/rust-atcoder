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

    let mut c = vec![2; n];
    for u in 0..n {
        if c[u] != 2 {
            continue;
        }

        let mut deque = VecDeque::new();
        c[u] = 0;
        deque.push_back(u);
        while let Some(u) = deque.pop_front() {
            for v in edges[u].iter().copied() {
                let c_v = if c[u] == 0 { 1 } else { 0 };
                if c[v] == 2 {
                    c[v] = c_v;
                    deque.push_back(v);
                } else if c[v] != c_v {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
