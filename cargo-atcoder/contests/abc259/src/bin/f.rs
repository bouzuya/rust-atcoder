use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uvw: &[(usize, usize, i64)]) -> Vec<Vec<(usize, i64)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn dfs(
    dp: &mut Vec<(i64, i64)>,
    inf: i64,
    d: &[usize],
    edges: &[Vec<(usize, i64)>],
    u: usize,
    p: usize,
) {
    let mut pq = BinaryHeap::new();
    for (v, w) in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(dp, inf, d, edges, v, u);
        dp[u].0 += dp[v].1;
        dp[u].1 += dp[v].1;

        pq.push((dp[v].0 + w) - (dp[v].1));
    }

    let mut count = 0;
    while let Some(diff) = pq.pop() {
        if diff <= 0 {
            break;
        }
        if count < d[u].saturating_sub(1) {
            dp[u].0 += diff;
        }
        if count < d[u] {
            dp[u].1 += diff;
        }
        count += 1;
    }
    if d[u] == 0 {
        dp[u].0 = -inf;
    }
}

fn main() {
    input! {
        n: usize,
        d: [usize; n],
        uvw: [(Usize1, Usize1, i64); n - 1],
    };

    let edges = adjacency_list(n, &uvw);

    let inf = 1_i64 << 40;
    // (ある頂点とその親を結ぶ辺を選ぶ場合, 選ばない場合)
    let mut dp = vec![(0, 0); n];
    dfs(&mut dp, inf, &d, &edges, 0, 0);

    let ans = dp[0].1;
    println!("{}", ans);
}
