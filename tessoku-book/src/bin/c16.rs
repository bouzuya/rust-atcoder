use std::collections::{BTreeMap, BTreeSet};

use proconio::{input, marker::Usize1};

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        asbt: [(Usize1, usize, Usize1, usize); m],
    };

    let max_t = 1_000_000_000 + k;
    let start = n;
    let goal = n + 1;
    let mut vertices = vec![];
    for (a, s, b, t) in asbt.iter().copied() {
        vertices.push((a, s));
        vertices.push((b, t + k));
    }
    for i in 0..n {
        vertices.push((i, 0));
    }
    vertices.push((start, 0));
    vertices.push((goal, max_t + 1));
    let map = vertices
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });

    let mut ts = vec![BTreeSet::new(); n];
    let mut times = vec![vec![]; n];
    for (a, s, b, t) in asbt.iter().copied() {
        ts[a].insert(s);
        ts[b].insert(t + k);
    }
    for i in 0..n {
        ts[i].insert(0);
        times[i].extend(&ts[i]);
    }

    let mut edges = vec![vec![]; map.len()];
    for (a, s, b, t) in asbt {
        let u = map[&(a, s)];
        let v = map[&(b, t + k)];
        edges[v].push((u, 1));
    }
    vertices.sort();
    for i in 0..n {
        let l = times[i].len();
        for j in 0..l - 1 {
            let u = map[(&(i, times[i][j]))];
            let v = map[&(i, times[i][j + 1])];
            edges[v].push((u, 0));
        }
        let u = map[(&(i, times[i][l - 1]))];
        let v = map[&(goal, max_t + 1)];
        edges[v].push((u, 0));
    }
    for i in 0..n {
        let u = map[&(start, 0)];
        let v = map[&(i, 0)];
        edges[v].push((u, 0));
    }

    let mut flat_times = vec![];
    for i in 0..n {
        flat_times.extend(
            times[i]
                .iter()
                .copied()
                .map(|t| (i, t))
                .collect::<Vec<(usize, usize)>>(),
        );
    }

    flat_times.sort_by_key(|&(_, t)| t);

    let mut dp = vec![0_usize; map.len()];
    for (i, t) in flat_times {
        let u = map[&(i, t)];
        for (v, w) in edges[u].iter().copied() {
            chmax!(dp[u], dp[v] + w);
        }
    }
    let u = map[&(goal, max_t + 1)];
    for (v, w) in edges[u].iter().copied() {
        chmax!(dp[u], dp[v] + w);
    }

    let ans = dp[u];
    println!("{}", ans);
}
