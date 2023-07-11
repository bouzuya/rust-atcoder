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
        w: usize,
        wv: [(usize, usize); n],
    };
    let w_1 = wv[0].0;
    let mut dp = vec![vec![vec![0_usize; 4 * n + 1]; n + 1]; n + 1];
    for (i, (w_i, v_i)) in wv.iter().copied().enumerate() {
        for j in 0..n {
            for k in 0..4 * n {
                chmax!(dp[i + 1][j][k], dp[i][j][k]);
                if (j + 1) * w_1 + k + (w_i - w_1) <= w && k + (w_i - w_1) <= 4 * n {
                    chmax!(dp[i + 1][j + 1][k + (w_i - w_1)], dp[i][j][k] + v_i);
                }
            }
        }
    }
    let mut ans = 0_usize;
    for j in 0..=n {
        for k in 0..4 * n {
            ans = ans.max(dp[n][j][k]);
        }
    }
    println!("{}", ans);
}
