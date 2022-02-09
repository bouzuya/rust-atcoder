use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };
    let inf = 1_000_000_000_i64;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n {
        if i > 0 {
            dp[i] = dp[i].min((h[i] - h[i - 1]).abs() + dp[i - 1]);
        }
        if i > 1 {
            dp[i] = dp[i].min((h[i] - h[i - 2]).abs() + dp[i - 2]);
        }
    }

    let ans = dp[n - 1];
    println!("{}", ans);
}
