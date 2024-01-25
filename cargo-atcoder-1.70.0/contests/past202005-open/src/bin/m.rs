use std::vec;

use proconio::{input, marker::Usize1};

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn dijkstra(n: usize, inf: usize, e: &[Vec<usize>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for v in e[u].iter().copied() {
            let w = w_u + 1;
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
        uv: [(Usize1, Usize1); m],
        s: Usize1,
        k: usize,
        t: [Usize1; k],
    };

    let inf = 1 << 60;
    let dist = {
        let mut edges = vec![vec![]; n];
        for (u, v) in uv {
            edges[u].push(v);
            edges[v].push(u);
        }
        let mut dist = vec![vec![0_usize; k + 1]; k + 1];
        for i in 0..=k {
            let d = dijkstra(n, inf, &edges, if i == k { s } else { t[i] });
            for j in 0..k {
                dist[i][j] = d[t[j]];
                dist[j][i] = d[t[j]];
            }
        }
        dist
    };

    let mut dp = vec![vec![inf; k + 1]; 1 << (k + 1)];
    dp[0][k] = 0;
    for i in 0..=k {
        chmin!(dp[1 << i][i], dist[k][i]);
    }
    for bits in 1..1 << (k + 1) {
        for i in 0..=k {
            if (bits & (1 << i)) != 0 {
                for j in 0..=k {
                    chmin!(dp[bits][j], dp[bits ^ (1 << i)][i] + dist[i][j]);
                }
            }
        }
    }
    let ans = dp[(1 << (k + 1)) - 1].iter().copied().min().unwrap();
    println!("{}", ans);
}
