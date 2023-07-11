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

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<(usize, u64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push((v, 1));
        e[v].push((u, 1));
    }
    e
}

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &(v, w_v) in e[u].iter() {
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
        ab: [(Usize1, Usize1); m],
        k: usize,
        c: [Usize1; k],
    };
    let inf = 1_000_000_000;
    let e = {
        let e = adjacency_list(n, &ab);
        let mut ec = vec![vec![]; k];
        for (i, &c_i) in c.iter().enumerate() {
            let d = dijkstra(n, inf, &e, c_i);
            for (j, &c_j) in c.iter().enumerate() {
                if i == j || d[c_j] == inf {
                    continue;
                }
                ec[i].push((j, d[c_j]));
            }
        }
        ec
    };
    let mut d = vec![];
    for s in 0..k {
        d.push(dijkstra(k, inf, &e, s));
    }

    let mut dp = vec![vec![inf; k]; 1 << k];
    for j in 0..k {
        dp[1 << j][j] = 0;
    }
    for i in 0..1 << k {
        for u in 0..k {
            for v in 0..k {
                if u == v {
                    continue;
                }
                chmin!(dp[i | 1 << v][v], dp[i][u] + d[u][v]);
            }
        }
    }
    let x = *dp[(1 << k) - 1].iter().min().unwrap();
    let ans = if x == inf { -1 } else { x as i64 + 1 };
    println!("{}", ans);
}
