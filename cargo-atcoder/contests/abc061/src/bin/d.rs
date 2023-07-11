use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };
    let abc = abc
        .iter()
        .map(|&(a_i, b_i, c_i)| (a_i, b_i, -c_i))
        .collect::<Vec<_>>();
    let inf = 1_000_000_000_000_000_000_i64;
    // bellman_ford
    let mut d = vec![inf; n];
    d[0] = 0_i64;
    for _ in 0..n - 1 {
        for &(u, v, w) in abc.iter() {
            if d[u] != inf && d[u] + w < d[v] {
                d[v] = d[u] + w;
            }
        }
    }
    let ans = -d[n - 1];
    let mut c = vec![false; n];
    for _ in 0..n {
        for &(u, v, w) in abc.iter() {
            if d[u] == inf {
                continue;
            }
            if d[u] + w < d[v] {
                d[v] = d[u] + w;
                c[v] = true;
            }
            if c[u] {
                c[v] = true;
            }
        }
    }
    if c[n - 1] {
        println!("inf");
    } else {
        println!("{}", ans);
    }
}
