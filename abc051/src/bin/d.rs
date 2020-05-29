use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uvw: &Vec<(usize, usize, i64)>) -> Vec<Vec<(usize, i64)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn bellman_ford(n: usize, inf: i64, e: &Vec<Vec<(usize, i64)>>, s: usize) -> Option<Vec<i64>> {
    let mut d = vec![inf; n];
    d[s] = 0;
    for i in 0..n {
        for (u, e_u) in e.iter().enumerate() {
            for &(v, w) in e_u.iter() {
                if d[u] + w < d[v] {
                    d[v] = d[u] + w;
                    if i == n - 1 {
                        return None;
                    }
                }
            }
        }
    }
    Some(d)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };

    let e = adjacency_list(n, &abc);
    let inf = 1_000_000_000;
    // d[u][v]: u から v への最短距離
    let mut d = vec![];
    for u in 0..n {
        match bellman_ford(n, inf, &e, u) {
            Some(d_u) => d.push(d_u),
            None => unreachable!(),
        }
    }
    let mut ans = 0;
    for &(u, v, c) in abc.iter() {
        if d[u][v] != c {
            ans += 1;
        }
    }
    println!("{}", ans);
}
