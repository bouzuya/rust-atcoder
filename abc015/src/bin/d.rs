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
        w: usize,
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    };
    let mut dp = vec![vec![vec![0; k + 1]; w + 1]; n + 1];
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        for w_i in 0..=w {
            for k_i in 0..=k {
                chmax!(dp[i + 1][w_i][k_i], dp[i][w_i][k_i]);
                if w_i + a_i <= w && k_i + 1 <= k {
                    chmax!(dp[i + 1][w_i + a_i][k_i + 1], dp[i][w_i][k_i] + b_i);
                }
            }
        }
    }
    let ans = dp[n][w][k];
    println!("{}", ans);
}
