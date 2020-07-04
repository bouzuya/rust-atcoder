use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    };

    let mut dp = vec![vec![0_f64; n + 1]; n + 1];
    dp[0][0] = 1_f64;
    for i in 0..n {
        for j in 0..n {
            dp[i + 1][j] += (1_f64 - p[i]) * dp[i][j];
            dp[i + 1][j + 1] += p[i] * dp[i][j];
        }
    }
    let mut ans = 0_f64;
    for (i, &a_i) in dp[n].iter().enumerate() {
        if i > n / 2 {
            ans += a_i;
        }
    }
    println!("{}", ans);
}
