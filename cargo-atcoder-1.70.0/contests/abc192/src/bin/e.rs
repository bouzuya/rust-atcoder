use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        abtk: [(Usize1, Usize1, usize, usize); m]
    };

    let mut edges = vec![vec![]; n];
    for (a, b, t, k) in abtk {
        edges[a].push((b, t, k));
        edges[b].push((a, t, k));
    }

    let inf = 1_usize << 60;
    let mut time = vec![inf; n];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0_usize), x));
    time[x] = 0_usize;
    while let Some((Reverse(t), u)) = pq.pop() {
        if t > time[u] {
            continue;
        }
        for (v, t, k) in edges[u].iter().copied() {
            let nt = if time[u] % k == 0 {
                time[u]
            } else {
                time[u] + k - (time[u] % k)
            } + t;
            if nt < time[v] {
                time[v] = nt;
                pq.push((Reverse(nt), v));
            }
        }
    }
    let ans = time[y];
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
