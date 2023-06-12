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
        m: usize,
        n: usize,
        s: Chars,
        t: Chars,
    }

    let inf = n * m + 1;
    let mut dp = vec![vec![inf; n + 1]; m + 1];
    dp[0][0] = 0_usize;
    for i in 0..=m {
        for j in 0..=n {
            if i > 0 && j > 0 {
                chmin!(
                    dp[i][j],
                    dp[i - 1][j - 1] + if s[i - 1] == t[j - 1] { 0 } else { 1 }
                );
            }
            if i > 0 {
                chmin!(dp[i][j], dp[i - 1][j] + 1);
            }
            if j > 0 {
                chmin!(dp[i][j], dp[i][j - 1] + 1);
            }
        }
    }
    let ans = dp[m][n];
    println!("{}", ans);
}
