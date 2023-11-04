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
        p: [f64; n],
    };

    let inf = 1e18_f64;
    let mut dp = vec![vec![-inf; n + 1]; n + 1];
    dp[0][0] = 0_f64;
    for (i, p_i) in p.iter().copied().enumerate() {
        for j in 0..=i {
            chmax!(dp[i + 1][j + 1], dp[i][j] * 0.9 + p_i);
            chmax!(dp[i + 1][j], dp[i][j]);
        }
    }

    let mut ans = -inf;
    for k in 1..=n {
        let mut x = 0_f64;
        let mut y = 1_f64;
        for _ in 0..k {
            x += y;
            y *= 0.9;
        }
        chmax!(ans, dp[n][k] / x - (1200_f64 / (k as f64).sqrt()));
    }
    println!("{}", ans);
}
