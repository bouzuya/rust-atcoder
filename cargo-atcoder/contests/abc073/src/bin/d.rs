use proconio::{input, marker::Usize1};
use superslice::Ext;

fn dijkstra(n: usize, inf: usize, edges: &[Vec<(usize, usize)>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for (v, w_v) in edges[u].iter().copied() {
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        capital_n: usize,
        capital_m: usize,
        capital_r: usize,
        mut r: [Usize1; capital_r],
        abc: [(Usize1, Usize1, usize); capital_m],
    };

    let mut edges = vec![vec![]; capital_n];
    for (a, b, c) in abc.iter().copied() {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }

    let inf = 1_usize << 60;

    let mut dist = vec![];
    for u in 0..capital_n {
        dist.push(dijkstra(capital_n, inf, &edges, u));
    }

    let mut min = 1_usize << 60;
    r.sort();
    loop {
        let mut sum = 0_usize;
        let mut prev = r[0];
        for next in r.iter().copied().skip(1) {
            sum += dist[prev][next];
            prev = next;
        }
        min = min.min(sum);
        if !r.next_permutation() {
            break;
        }
    }

    let ans = min;
    println!("{}", ans);
}
