use std::{
    collections::{BTreeMap, BTreeSet},
    iter,
};

use proconio::input;

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
        asbt: [(usize, usize, usize, usize); m],
    };
    let asbt = asbt
        .iter()
        .copied()
        .map(|(a, s, b, t)| (a, s + 1, b, t + k + 1))
        .collect::<Vec<_>>();

    let max_t = 1_000_000_000 + k;
    let start = 0;
    let goal = n + 1;
    let vertices = asbt
        .iter()
        .copied()
        .map(|(a, s, _, _)| (s, a))
        .chain(asbt.iter().copied().map(|(_, _, b, t)| (t, b)))
        .chain(iter::once((0, start)))
        .chain(iter::once((max_t + 1, goal)))
        .collect::<BTreeSet<_>>();
    let map = vertices
        .iter()
        .copied()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });

    let mut times = vec![BTreeSet::new(); n + 1];
    for (a, s, b, t) in asbt.iter().copied() {
        times[a].insert(s);
        times[b].insert(t);
    }

    let mut edges = vec![vec![]; map.len()];
    for (a, s, b, t) in asbt {
        let u = map[&(s, a)];
        let v = map[&(t, b)];
        edges[v].push((u, 1));
    }
    for i in 1..=n {
        let ts = times[i].iter().copied().collect::<Vec<usize>>();
        if ts.is_empty() {
            continue;
        }

        let u = map[&(0, start)];
        let v = map[&(*ts.first().unwrap(), i)];
        edges[v].push((u, 0));

        for t in ts.windows(2) {
            let u = map[&(t[0], i)];
            let v = map[&(t[1], i)];
            edges[v].push((u, 0));
        }

        let u = map[&(*ts.last().unwrap(), i)];
        let v = map[&(max_t + 1, goal)];
        edges[v].push((u, 0));
    }

    let mut ans = 0_usize;
    let mut dp = vec![0_usize; map.len()];
    for (t, i) in vertices {
        let v = map[&(t, i)];
        for (u, w) in edges[v].iter().copied() {
            chmax!(dp[v], dp[u] + w);
        }
        ans = ans.max(dp[v]);
    }

    println!("{}", ans);
}
