use std::{
    cmp,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uvw: &[(usize, usize, u64)]) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abc: [(Usize1, Usize1, u64); m],
        x: [u64; q],
    };

    let e = adjacency_list(n, &abc);
    let mut heap = BinaryHeap::new();
    let mut set = BTreeSet::new();
    set.insert(0);
    for &(v, w) in e[0].iter() {
        heap.push((cmp::Reverse(w), v));
    }
    for x_i in x {
        let mut next = vec![];
        while let Some((cmp::Reverse(w), v)) = heap.pop() {
            if w > x_i {
                heap.push((cmp::Reverse(w), v));
                break;
            }
            if set.insert(v) {
                for &(nv, nw) in e[v].iter() {
                    next.push((cmp::Reverse(nw), nv));
                }
            }
        }
        for x in next {
            heap.push(x);
        }
        println!("{}", set.len());
    }
}
