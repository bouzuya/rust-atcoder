use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut dp = vec![0; std::cmp::max(n, 3)];
    dp[0] = 0;
    dp[1] = 0;
    dp[2] = 1;
    for i in 3..n {
        dp[i] = (dp[i - 1] + dp[i - 2] + dp[i - 3]) % 10_007;
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
