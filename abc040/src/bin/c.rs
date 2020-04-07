use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [isize; n],
    };
    let mut dp = vec![1_000_000_000_000_isize; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[i + 1] = std::cmp::min(dp[i + 1], dp[i] + (av[i + 1] - av[i]).abs());
        if i + 2 < n {
            dp[i + 2] = std::cmp::min(dp[i + 2], dp[i] + (av[i + 2] - av[i]).abs());
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
