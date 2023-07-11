use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut edges = vec![vec![]; n];
    for a_i in a {
        let u = a_i;
        let v = a_i + 1;
        edges[u].push(v);
        edges[v].push(u);
    }

    let mut used = vec![false; n];
    for u in 0..n {
        if used[u] {
            continue;
        }

        used[u] = true;
        let mut vs = vec![u];
        let mut deque = VecDeque::new();
        deque.push_back(u);
        while let Some(u) = deque.pop_front() {
            for v in edges[u].iter().copied() {
                if !used[v] {
                    used[v] = true;
                    vs.push(v);
                    deque.push_back(v);
                }
            }
        }
        vs.sort();
        vs.reverse();

        for v in vs {
            println!("{}", v + 1);
        }
    }
}
