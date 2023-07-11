use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    };
    let map = ab
        .iter()
        .copied()
        .map(|(a, _)| a)
        .chain(ab.iter().copied().map(|(_, b)| b))
        .chain(std::iter::once(0))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });
    let mut rev = HashMap::new();
    for (k, v) in map.iter() {
        rev.insert(v, k);
    }

    let mut edges = vec![vec![]; map.len()];
    for (u, v) in ab.iter().copied() {
        edges[map[&u]].push(map[&v]);
        edges[map[&v]].push(map[&u]);
    }

    let mut max = 0_usize;
    let mut used = HashSet::new();
    let mut deque = VecDeque::new();
    used.insert(0);
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if !used.insert(v) {
                continue;
            }
            deque.push_back(v);
            max = max.max(v);
        }
    }

    let ans = *rev.get(&max).unwrap() + 1;
    println!("{}", ans);
}
