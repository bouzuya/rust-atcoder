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
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for (i, &(w_i, v_i)) in wv.iter().enumerate() {
        for j in 0..w {
            chmax!(dp[i + 1][j], dp[i][j]);
            if j + 1 <= w {
                chmax!(dp[i + 1][j + 1], dp[i][j]);
            }
            if j + w_i <= w {
                chmax!(dp[i + 1][j + w_i], dp[i][j] + v_i);
            }
        }
    }
    let ans = dp[n][w];
    println!("{}", ans);
}
