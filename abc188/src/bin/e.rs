// 解説 AC
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

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

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        xy: [(Usize1, Usize1); m],
    };
    let mut e = vec![vec![]; n];
    for &(x_i, y_i) in xy.iter() {
        e[x_i].push(y_i);
    }

    let inf = 1_000_000_000_000;
    let mut dp = vec![inf; n];
    let mut ans = -inf;
    for u in 0..n {
        chmax!(ans, a[u] - dp[u]);
        for &v in e[u].iter() {
            chmin!(dp[v], min(a[u], dp[u]));
        }
    }
    println!("{}", ans);
}
