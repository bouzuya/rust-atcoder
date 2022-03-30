use std::collections::{BTreeSet, VecDeque};

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for (i, (u, v)) in uv.iter().copied().enumerate() {
        e[u].push((v, i));
        e[v].push((u, i));
    }
    e
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let edges = adjacency_list(n, &ab);
    let inf = 1_000_000_000;
    let mut colors = vec![inf; n - 1];
    let mut deque = VecDeque::new();
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        let mut c = 1;
        let mut s = BTreeSet::new();
        for (_, i) in edges[u].iter().copied() {
            if colors[i] != inf {
                s.insert(colors[i]);
                continue;
            }
        }
        for (v, i) in edges[u].iter().copied() {
            if colors[i] != inf {
                continue;
            }
            while !s.insert(c) {
                c += 1;
            }
            colors[i] = c;
            deque.push_back(v);
        }
    }

    println!("{}", colors.iter().max().unwrap());
    for c in colors {
        println!("{}", c);
    }
}
