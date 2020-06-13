use proconio::input;
use proconio::marker::Usize1;

macro_rules! chmin {
    ($e: expr, $v: expr) => {
        if $v < $e {
            $e = $v;
        }
    };
}

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn bfs(n: usize, inf: u64, e: &Vec<Vec<usize>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut q = std::collections::VecDeque::new();
    d[s] = 0;
    q.push_back((0, s));
    while let Some((w_u, u)) = q.pop_front() {
        if w_u > d[u] {
            continue;
        }
        for &v in e[u].iter() {
            let w = w_u + 1;
            if w < d[v] {
                d[v] = w;
                q.push_back((w, v));
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

    let inf = 1_000_000_000_000;
    let d_t = {
        let mut d_t = vec![vec![inf; k + 1]; k + 1];
        let e = adjacency_list(n, &uv);
        let d_s = bfs(n, inf, &e, s);
        for i in 0..k {
            chmin!(d_t[i + 1][0], d_s[t[i]]);
            chmin!(d_t[0][i + 1], d_s[t[i]]);
        }
        for (i, &t_i) in t.iter().enumerate() {
            let d_t_i = bfs(n, inf, &e, t_i);
            for (j, &t_j) in t.iter().enumerate() {
                chmin!(d_t[i + 1][j + 1], d_t_i[t_j]);
                chmin!(d_t[j + 1][i + 1], d_t_i[t_j]);
            }
        }
        d_t
    };
    let mut dp = vec![vec![inf; k + 1]; 1 << (k + 1)];
    dp[0][0] = 0;
    for i in 0..1 << (k + 1) {
        for u in 0..k + 1 {
            for v in 0..k + 1 {
                chmin!(dp[i | 1 << u][v], dp[i][u] + d_t[u][v]);
            }
        }
    }
    let mut mask = 0;
    for i in 0..k + 1 {
        mask |= 1 << i;
    }
    println!("{}", dp[mask][k]);
}
