use proconio::{input, marker::Chars};

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
        c: [usize; n],
        d: [usize; n],
    }

    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    dp[0][0] = 0_usize;
    for (i, s_i) in s.iter().copied().enumerate() {
        for j in 0..n {
            match s_i {
                '(' => {
                    chmin!(dp[i + 1][j + 1], dp[i][j]);
                    if j > 0 {
                        chmin!(dp[i + 1][j - 1], dp[i][j] + c[i]);
                    }
                    chmin!(dp[i + 1][j], dp[i][j] + d[i]);
                }
                ')' => {
                    if j > 0 {
                        chmin!(dp[i + 1][j - 1], dp[i][j]);
                    }
                    chmin!(dp[i + 1][j], dp[i][j] + d[i]);
                    chmin!(dp[i + 1][j + 1], dp[i][j] + c[i]);
                }
                _ => unreachable!(),
            }
        }
    }
    let ans = dp[n][0];
    println!("{}", ans);
}
