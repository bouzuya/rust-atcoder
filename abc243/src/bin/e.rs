use proconio::{input, marker::Usize1};
fn adjacency_list(n: usize, uvw: &[(usize, usize, i64)]) -> Vec<Vec<(usize, i64)>> {
    let mut e = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn warshall_floyd(n: usize, inf: i64, e: &[Vec<(usize, i64)>]) -> Vec<Vec<i64>> {
    let mut d = vec![vec![inf; n]; n];
    for (u, e_u) in e.iter().enumerate() {
        d[u][u] = inf;
        for &(v, w) in e_u.iter() {
            d[u][v] = w;
        }
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                if d[u][k] + d[k][v] < d[u][v] {
                    d[u][v] = d[u][k] + d[k][v];
                }
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };

    let edges = adjacency_list(n, &abc);
    let inf = 1 << 60;
    let dist = warshall_floyd(n, inf, &edges);
    let mut count = 0_usize;
    for (a, b, c) in abc {
        if (0..n).any(|i| dist[a][i] + dist[i][b] <= c) {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
