use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b) in a.into_iter().zip(b.into_iter()) {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut color = vec![2; n];
    for start in 0..n {
        if color[start] != 2 {
            continue;
        }
        let mut q = VecDeque::new();
        q.push_back(start);
        color[start] = 0;
        while let Some(u) = q.pop_front() {
            for v in edges[u].iter().copied() {
                if color[v] == 2 {
                    color[v] = 1 - color[u];
                    q.push_back(v);
                } else if color[v] == color[u] {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
