use proconio::input;

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
        a: [usize; n],
        b: [usize; m],
    };

    let inf = 1_000_000_000;
    let mut dp = vec![vec![inf; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..=n {
        for j in 0..=m {
            if i < n {
                chmin!(dp[i + 1][j], dp[i][j] + 1);
            }
            if j < m {
                chmin!(dp[i][j + 1], dp[i][j] + 1);
            }
            if i < n && j < m {
                chmin!(
                    dp[i + 1][j + 1],
                    dp[i][j] + if a[i] == b[j] { 0 } else { 1 }
                );
            }
        }
    }

    let ans = dp[n][m];
    println!("{}", ans);
}
