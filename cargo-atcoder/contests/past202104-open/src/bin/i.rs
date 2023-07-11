use proconio::input;

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

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };
    // dp[i][j][k] = (i, j) までに k 匹を釣ったときのおいしさの総和の最大値
    let mut dp = vec![vec![vec![0_usize; h + w + 2]; w + 1]; h + 1];
    dp[0][0][0] = 0;
    dp[0][0][1] = a[0][0];
    for i in 0..h {
        for j in 0..w {
            for k in 0..=h + w {
                chmax!(dp[i + 1][j][k], dp[i][j][k]);
                if i + 1 < h {
                    chmax!(dp[i + 1][j][k + 1], dp[i][j][k] + a[i + 1][j]);
                }
                chmax!(dp[i][j + 1][k], dp[i][j][k]);
                if j + 1 < w {
                    chmax!(dp[i][j + 1][k + 1], dp[i][j][k] + a[i][j + 1]);
                }
            }
        }
    }
    for k in 1..h + w {
        let ans = dp[h - 1][w - 1][k];
        println!("{}", ans);
    }
}
