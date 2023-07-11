use std::collections::{BTreeSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut rev = vec![vec![]; n];
    let mut edges = vec![BTreeSet::new(); n];
    for (u, v) in uv.iter().copied() {
        edges[u].insert(v);
        rev[v].push(u);
    }

    let mut deque = edges
        .iter()
        .enumerate()
        .filter(|(_, e)| e.is_empty())
        .map(|(v, _)| v)
        .collect::<VecDeque<usize>>();
    let mut used = vec![false; n];
    while let Some(v) = deque.pop_front() {
        if used[v] {
            continue;
        }
        used[v] = true;

        for u in rev[v].iter().copied() {
            edges[u].remove(&v);
            if edges[u].is_empty() {
                deque.push_back(u);
            }
        }
    }

    let ans = used.into_iter().filter(|&b| !b).count();
    println!("{}", ans);
}
