use proconio::{input, marker::Usize1};

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
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    };

    let mut edges = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        edges[u].push((v, w));
    }

    let mut rev_edges = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        rev_edges[v].push((u, w));
    }

    let inf = 1_usize << 60;
    let dist = dijkstra(n, inf, &edges, 0);
    let rev_dist = dijkstra(n, inf, &rev_edges, n - 1);

    for (d, r) in dist.into_iter().zip(rev_dist.into_iter()) {
        let a = if d == inf || r == inf {
            -1
        } else {
            (d + r) as i64
        };
        println!("{}", a);
    }
}
