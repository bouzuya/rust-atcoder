use proconio::input;
use proconio::marker::Usize1;

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

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<u64>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for (v, &w_v) in e[u].iter().enumerate() {
            if u == v || w_v == inf {
                continue;
            }
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(std::cmp::Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m],
        k: usize,
        xyz: [(Usize1, Usize1, u64); k],
    };

    let inf = 1_000_000_000;
    let e = {
        let mut e = vec![vec![inf; n]; n];
        for &(u, v, w) in abc.iter() {
            e[u][v] = w;
            e[v][u] = w;
        }
        e
    };
    let mut d = (0..n)
        .map(|i| dijkstra(n, inf, &e, i))
        .collect::<Vec<Vec<u64>>>();
    for &(u, v, w) in xyz.iter() {
        for i in 0..n {
            for j in i + 1..n {
                let d_ij = std::cmp::min(d[i][u] + w + d[v][j], d[i][v] + w + d[u][j]);
                chmin!(d[i][j], d_ij);
                chmin!(d[j][i], d_ij);
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                ans += d[i][j];
            }
        }
        println!("{}", ans);
    }
}
