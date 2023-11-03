use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        q: usize,
        tex: [(usize, Usize1, i64); q],
    }

    let mut edges = vec![vec![]; n];
    for (a, b) in ab.iter().copied() {
        edges[a].push(b);
        edges[b].push(a);
    }

    let depth = {
        let inf = n + 1;
        let mut depth = vec![inf; n];
        let mut q = VecDeque::new();
        depth[0] = 0;
        q.push_back(0);
        while let Some(u) = q.pop_front() {
            for v in edges[u].iter().copied() {
                if depth[v] != inf {
                    continue;
                }
                depth[v] = depth[u] + 1;
                q.push_back(v);
            }
        }
        depth
    };

    let mut c = vec![0_i64; n];

    for (t, e, x) in tex {
        let (a, b) = ab[e];
        let (p, u) = if depth[a] < depth[b] {
            (t == 1, b)
        } else {
            (t == 2, a)
        };
        if p {
            c[0] += x;
            c[u] -= x;
        } else {
            c[u] += x;
        }
    }

    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for v in edges[u].iter().copied() {
            if depth[v] <= depth[u] {
                continue;
            }
            c[v] += c[u];
            q.push_back(v);
        }
    }

    for c_i in c {
        println!("{}", c_i);
    }
}
