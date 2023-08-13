use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        c: [Usize1; k],
    };

    let inf = 1_usize << 60;

    let mut edges = vec![vec![]; n];
    for (a, b) in ab.iter().copied() {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut dists = vec![];
    for c_i in c.iter().copied() {
        let mut deque = VecDeque::new();
        let mut dist = vec![inf; n];
        deque.push_back(c_i);
        dist[c_i] = 0;
        while let Some(u) = deque.pop_front() {
            for v in edges[u].iter().copied() {
                if dist[v] != inf {
                    continue;
                }
                dist[v] = dist[u] + 1;
                deque.push_back(v);
            }
        }
        dists.push(dist);
    }

    let mut dist = vec![vec![inf; k]; k];
    for i in 0..k {
        for j in 0..k {
            dist[i][j] = dists[i][c[j]];
        }
    }

    let mut dp = vec![vec![inf; k]; 1 << k];
    for i in 0..k {
        dp[1 << i][i] = 0_usize;
    }
    for bits in 1..1 << k {
        for u in 0..k {
            if (bits >> u) & 1 == 0 {
                continue;
            }
            for v in 0..k {
                if (bits >> v) & 1 == 1 {
                    continue;
                }
                dp[bits | (1 << v)][v] = dp[bits | (1 << v)][v].min(dp[bits][u] + dist[u][v]);
            }
        }
    }
    let min = *dp[(1 << k) - 1].iter().min().unwrap();
    let ans = if min == inf { -1 } else { (min + 1) as i64 };
    println!("{}", ans);
}
