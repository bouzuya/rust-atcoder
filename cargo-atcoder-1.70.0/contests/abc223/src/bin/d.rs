use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn topological_sort(e: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = e.len();
    let mut c_in = vec![0; n];
    for e_u in e.iter() {
        for &v in e_u.iter() {
            c_in[v] += 1;
        }
    }
    // u の小さいものを取り出す
    let mut pq = BinaryHeap::new();
    for u in 0..n {
        if c_in[u] == 0 {
            pq.push(Reverse(u));
        }
    }
    let mut res = vec![];
    while let Some(Reverse(u)) = pq.pop() {
        res.push(u);
        for &v in e[u].iter() {
            c_in[v] -= 1;
            if c_in[v] == 0 {
                pq.push(Reverse(v));
            }
        }
    }
    if res.len() == n {
        Some(res)
    } else {
        None
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
    }

    if let Some(sorted) = topological_sort(&edges) {
        for u in sorted {
            println!("{}", u + 1);
        }
    } else {
        println!("-1");
    }
}
