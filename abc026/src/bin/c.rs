use proconio::input;
use proconio::marker::Usize1;
use std::cmp::{max, min};

fn dfs(e: &Vec<Vec<usize>>, u: usize, res: &mut Vec<usize>) {
    let mut max_s = None;
    let mut min_s = None;
    for &v in e[u].iter() {
        dfs(e, v, res);
        let s = res[v];
        max_s = Some(max(max_s.unwrap_or(0), s));
        min_s = Some(min(min_s.unwrap_or(1_000_000_000), s));
    }
    res[u] = max_s.unwrap_or(0) + min_s.unwrap_or(0) + 1;
}

fn main() {
    input! {
        n: usize,
        b: [Usize1; n - 1],
    };

    let mut e = vec![vec![]; n];
    for (i, &b_i) in b.iter().enumerate() {
        e[b_i].push(i + 1);
    }

    let mut res = vec![0; n];
    dfs(&e, 0, &mut res);

    let ans = res[0];
    println!("{}", ans);
}
