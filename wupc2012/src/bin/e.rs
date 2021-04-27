use std::{cmp, collections::BinaryHeap};

use proconio::input;

fn adjacency_list(n: usize, uvw: &Vec<(usize, usize, u64)>) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn f(n: usize, e: &Vec<Vec<(usize, u64)>>, p: usize) -> Vec<Vec<u64>> {
    let inf = 1_000_000_000_u64;
    let mut d = vec![vec![inf; p]; n];
    d[0][0] = 0;
    let mut q = BinaryHeap::new();
    q.push((cmp::Reverse(0), 0));
    while let Some((cmp::Reverse(t), u)) = q.pop() {
        if t > d[u][t as usize % p] {
            continue;
        }
        if u == n - 1 {
            continue;
        }
        for &(v, v_w) in e[u].iter() {
            let nt = t + v_w;
            if nt < d[v][nt as usize % p] {
                d[v][nt as usize % p] = nt;
                q.push((cmp::Reverse(nt), v));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ftc: [(usize, usize, u64); m],
    };

    let e = adjacency_list(n, &ftc);
    let d4 = f(n, &e, 4);
    let d7 = f(n, &e, 7);
    let ans = cmp::min(d4[n - 1][0], d7[n - 1][0]);
    println!("{}", ans);
}
