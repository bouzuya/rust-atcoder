// 解説 AC <https://atcoder.jp/contests/abc210/editorial/2298>
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
        h: usize,
        w: usize,
        c: i64,
        mut a: [[i64; w]; h],
    };
    // dp[i][j] := 一方の駅をすでに建設して現在 (i,j) にいるとき、かかった費用として考えられる最小値
    let inf = 1_000_000_000_000_000_000_i64;
    let mut ans = inf;
    for _ in 0..2 {
        let mut dp = vec![vec![inf; w]; h];
        for i in 0..h {
            for j in 0..w {
                chmin!(dp[i][j], a[i][j]);
                if i > 0 {
                    chmin!(dp[i][j], dp[i - 1][j] + c);
                }
                if j > 0 {
                    chmin!(dp[i][j], dp[i][j - 1] + c);
                }
            }
        }
        for i in 0..h {
            for j in 0..w {
                if i > 0 {
                    chmin!(ans, dp[i - 1][j] + c + a[i][j]);
                }
                if j > 0 {
                    chmin!(ans, dp[i][j - 1] + c + a[i][j]);
                }
            }
        }
        a.reverse();
    }

    println!("{}", ans);
}
