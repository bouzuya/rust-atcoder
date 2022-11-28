use std::cmp::Reverse;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut edges = vec![0_usize; n];
    for (a, b) in ab {
        edges[a] += 1;
        edges[b] += 1;
    }

    let mut count = edges
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    count.sort_by_key(|&(_, c)| Reverse(c));

    let ans = count[0].0 + 1;
    println!("{}", ans);
}
