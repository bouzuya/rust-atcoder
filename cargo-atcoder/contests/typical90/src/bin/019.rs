// 区間 DP
// 解説 AC <https://twitter.com/e869120/status/1384638694162780166>
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
        a: [i64; 2 * n],
    };
    let inf = 1_000_000_000_i64;
    let mut dp = vec![vec![inf; 2 * n]; 2 * n];
    // l = 1
    for i in 0..2 * n - 1 {
        dp[i][i + 1] = (a[i] - a[i + 1]).abs();
    }
    for len in (3..2 * n).step_by(2) {
        for l in 0..2 * n - len {
            let r = l + len;
            chmin!(dp[l][r], dp[l + 1][r - 1] + (a[l] - a[r]).abs());
            for k in l + 1..r {
                chmin!(dp[l][r], dp[l][k] + dp[k + 1][r]);
            }
        }
    }
    let ans = dp[0][2 * n - 1];
    println!("{}", ans);
}
