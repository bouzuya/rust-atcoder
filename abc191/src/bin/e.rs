use std::cmp::min;

use proconio::input;
use proconio::marker::Usize1;

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &(v, w_v) in e[u].iter() {
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(std::cmp::Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m],
    };

    let inf = 1_000_000_000;

    let mut e_loop = vec![inf; n];
    let mut e_in = vec![vec![]; n];
    let mut e = vec![vec![]; n];
    for &(u, v, w) in abc.iter() {
        if u == v {
            e_loop[u] = min(e_loop[u], w);
            continue;
        }
        e[u].push((v, w));
        e_in[v].push((u, w));
    }

    let mut ds = vec![];
    for i in 0..n {
        let d = dijkstra(n, inf, &e, i);
        ds.push(d);
    }

    for v in 0..n {
        let mut ans = e_loop[v];
        for &(u, w) in e_in[v].iter() {
            ans = min(ans, ds[v][u] + w);
        }
        println!("{}", if ans == inf { -1 } else { ans as i64 });
    }
}
