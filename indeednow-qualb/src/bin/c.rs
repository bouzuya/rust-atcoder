use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };
    let e = adjacency_list(n, &ab);

    let mut b = vec![false; n];
    let mut ans = vec![];
    let mut pq = BinaryHeap::new();
    b[0] = true;
    pq.push(Reverse(0));
    while let Some(Reverse(u)) = pq.pop() {
        ans.push(u);
        for &v in e[u].iter() {
            if b[v] {
                continue;
            }
            b[v] = true;
            pq.push(Reverse(v));
        }
    }

    for (i, &a) in ans.iter().enumerate() {
        print!("{}{}", a + 1, if i == ans.len() - 1 { "\n" } else { " " });
    }
}
