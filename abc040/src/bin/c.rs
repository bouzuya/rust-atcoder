use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [isize; n],
    };
    let inf = 1_000_000_000_000_isize;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    dp[1] = (av[1] - av[0]).abs();
    for i in 2..n {
        dp[i] = std::cmp::min(
            dp[i],
            std::cmp::min(
                dp[i - 1] + (av[i] - av[i - 1]).abs(),
                dp[i - 2] + (av[i] - av[i - 2]).abs(),
            ),
        );
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
