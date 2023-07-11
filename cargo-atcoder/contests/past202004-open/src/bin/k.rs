use proconio::input;
use proconio::marker::Chars;

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
        s: Chars,
        c: [i64; n],
        d: [i64; n],
    };
    let inf = 1_000_000_000_000_000;
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=n {
            chmin!(dp[i + 1][j], dp[i][j] + d[i]);
        }
        for j in 0..n {
            if s[i] == '(' {
                chmin!(dp[i + 1][j + 1], dp[i][j]);
            } else {
                chmin!(dp[i + 1][j + 1], dp[i][j] + c[i]);
            }
        }
        for j in 1..=n {
            if s[i] == '(' {
                chmin!(dp[i + 1][j - 1], dp[i][j] + c[i]);
            } else {
                chmin!(dp[i + 1][j - 1], dp[i][j]);
            }
        }
    }
    let ans = dp[n][0];
    println!("{}", ans);
}
