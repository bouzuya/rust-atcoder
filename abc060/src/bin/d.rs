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
    let mut dp = vec![vec![vec![0; 400 + 1]; n + 1]; n + 1];
    let w_1 = wv[0].0;
    for (i, (w_i, v_i)) in wv.iter().enumerate() {
        for j in 0..=n {
            for k in 0..=400 {
                chmax!(dp[i + 1][j][k], dp[i][j][k]);
                if j + 1 <= n && k + w_i - w_1 <= 400 && w_1 * j + w_i + k <= w {
                    chmax!(dp[i + 1][j + 1][k + w_i - w_1], dp[i][j][k] + v_i);
                }
            }
        }
    }
    let mut ans = 0;
    for j in 0..=n {
        chmax!(ans, *dp[n][j].iter().max().unwrap());
    }
    println!("{}", ans);
}
