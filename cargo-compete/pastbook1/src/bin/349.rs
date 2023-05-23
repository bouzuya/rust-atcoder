use std::collections::VecDeque;

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

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
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
    }

    let edges = adjacency_list(n, &uv);

    let t = std::iter::once(s)
        .chain(t.into_iter())
        .collect::<Vec<usize>>();

    let inf = 1 << 60;
    let mut dist_t = vec![];
    for t_i in t.iter().copied() {
        let dist_t_i = dijkstra(n, inf, &edges, t_i);
        let dist_t_i = t
            .iter()
            .copied()
            .map(|t_j| dist_t_i[t_j])
            .collect::<Vec<_>>();
        dist_t.push(dist_t_i);
    }

    let k = k + 1;
    let mut dp = vec![vec![inf; k]; 1 << k];
    dp[1][0] = 0_usize;
    let mut deque = VecDeque::new();
    deque.push_back((1, 0));
    while let Some((set, u)) = deque.pop_front() {
        for v in 0..k {
            if (set & (1 << v)) != 0 {
                continue;
            }
            if chmin!(dp[set | (1 << v)][v], dp[set][u] + dist_t[u][v]) {
                deque.push_back((set | (1 << v), v));
            }
        }
    }
    let ans = dp[(1 << k) - 1].iter().min().unwrap();
    println!("{}", ans);
}
