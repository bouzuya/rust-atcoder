use proconio::input;
use proconio::marker::Bytes;
use std::cmp::min;

fn main() {
    input! {
        mut s: Bytes
    };
    let mut rv = vec![0usize; s.len() + 1];
    for i in 0..s.len() {
        rv[s.len() - i - 1] = (s[i] - b'0') as usize;
    }
    // println!("{:?}", rv);

    let mut dp = vec![vec![10_000_000; 2]; rv.len() + 1];
    dp[0][0] = 0;
    for i in 0..rv.len() {
        for j in 0..=1 {
            let d = rv[i] + j;
            for a in 0..=9 {
                let ni = i + 1;
                let nj = if a < d { 1 } else { 0 };
                let b = if a < d { 10 + a - d } else { a - d };
                dp[ni][nj] = min(dp[ni][nj], dp[i][j] + a + b);
            }
        }
    }
    let ans = dp[rv.len()][0];
    println!("{}", ans);
}
