use proconio::{input, marker::Usize1};

fn dijkstra(n: usize, inf: u64, e: &[Vec<(usize, u64)>], s: usize) -> Vec<u64> {
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
        k: usize,
        uva: [(Usize1, Usize1, usize); m],
        s: [Usize1; k],
    };
    let mut edges = vec![vec![]; 2 * n];
    for (u, v, a) in uva {
        edges[u + a * n].push((v + a * n, 1));
        edges[v + a * n].push((u + a * n, 1));
    }
    for s_i in s {
        edges[s_i].push((s_i + n, 0));
        edges[s_i + n].push((s_i, 0));
    }

    let inf = 1_u64 << 60;
    let d = dijkstra(2 * n, inf, &edges, n);
    let on = d[n - 1];
    let off = d[2 * n - 1];
    let ans = if on == inf && off == inf {
        -1
    } else {
        on.min(off) as i64
    };
    println!("{}", ans);
}
