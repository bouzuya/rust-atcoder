use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: [usize; n]
    };
    let max_x = 50;
    let mut dp = vec![vec![vec![0; n * max_x + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for (i, &x_i) in x.iter().enumerate() {
        for j in 0..n {
            for k in 0..=n * max_x {
                dp[i + 1][j][k] += dp[i][j][k];
                if k + x_i <= n * max_x {
                    dp[i + 1][j + 1][k + x_i] += dp[i][j][k];
                }
            }
        }
    }
    let mut sum = 0_usize;
    for i in 1..=n {
        sum += dp[n][i][i * a];
    }
    let ans = sum;
    println!("{}", ans);
}
