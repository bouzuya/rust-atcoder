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
        abc: [(Usize1, Usize1, usize); m]
    };

    let mut edges = vec![vec![]; n];
    for (a, b, c) in abc {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }

    let inf = 1 << 60;
    let dist0 = dijkstra(n, inf, &edges, 0);
    let distn = dijkstra(n, inf, &edges, n - 1);
    for k in 0..n {
        println!("{}", dist0[k] + distn[k]);
    }
}
