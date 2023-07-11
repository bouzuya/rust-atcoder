use proconio::input;

fn dijkstra(n: usize, inf: usize, e: &[Vec<(usize, usize)>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for (v, w_v) in e[u].iter().copied() {
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
        h: usize,
        w: usize,
        c: [[usize; 10]; 10],
        a: [[i64; w]; h],
    };

    let mut edges = vec![vec![]; 10];
    for i in 0..10 {
        for j in 0..10 {
            edges[i].push((j, c[i][j]));
        }
    }

    let mut dists = vec![];
    for i in 0..10 {
        let d = dijkstra(10, 1 << 60, &edges, i);
        dists.push(d[1]);
    }

    let mut ans = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == -1 {
                continue;
            }
            ans += dists[a[i][j] as usize];
        }
    }

    println!("{}", ans);
}
