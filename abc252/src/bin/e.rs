use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uvw: &[(usize, usize, u64, usize)]) -> Vec<Vec<(usize, u64, usize)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w, i) in uvw.iter().copied() {
        e[u].push((v, w, i));
        e[v].push((u, w, i));
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m],
    };

    let abci = abc
        .iter()
        .enumerate()
        .map(|(i, &(a, b, c))| (a, b, c, i))
        .collect::<Vec<(usize, usize, u64, usize)>>();
    let edges = adjacency_list(n, &abci);

    let inf = 1_u64 << 60;
    let mut ans = BTreeSet::new();
    let mut dist = vec![inf; n];
    let mut pq = BinaryHeap::new();
    dist[0] = 0;
    for (v, w, i) in edges[0].iter().copied() {
        dist[v] = w;
        pq.push(Reverse((w, v, i)));
        ans.insert(i);
    }
    while let Some(Reverse((w_u, u, i))) = pq.pop() {
        if w_u > dist[u] {
            ans.remove(&i);
            continue;
        }
        for (v, w_v, i) in edges[u].iter().copied() {
            let w = w_u + w_v;
            if w < dist[v] {
                dist[v] = w;
                pq.push(Reverse((w, v, i)));
                ans.insert(i);
            }
        }
    }
    for a in ans {
        println!("{}", a + 1);
    }
}
