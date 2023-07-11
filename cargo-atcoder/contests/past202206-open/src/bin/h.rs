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
        n: usize,
        a: usize,
        b: usize,
        wv: [(usize, usize); n],
    };

    let mut dp = vec![vec![vec![0_usize; b + 1]; a + 1]; n + 1];
    for (i, (w, v)) in wv.iter().copied().enumerate() {
        for j in 0..=a {
            for k in 0..=b {
                if j + w <= a {
                    chmax!(dp[i + 1][j + w][k], dp[i][j][k] + v);
                }
                if k + w <= b {
                    chmax!(dp[i + 1][j][k + w], dp[i][j][k] + v);
                }
                chmax!(dp[i + 1][j][k], dp[i][j][k]);
            }
        }
    }

    let mut max = 0_usize;
    for j in 0..=a {
        for k in 0..=b {
            chmax!(max, dp[n][j][k]);
        }
    }

    let ans = max;
    println!("{}", ans);
}
