use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [isize; n],
    };

    let inf = 1_000_000_000_000_000_005_isize;
    let mut dp = vec![vec![-inf; n + 1]; n + 1];
    dp[0][0] = 0;
    dp[1][0] = 0;
    dp[1][1] = av[0];
    for i in 2..=n {
        let a = av[i - 1];
        for j in (i / 2 - 1) + 1..=((i + 1) / 2) + 1 {
            dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 2][j - 1] + a);
        }
    }
    let ans = dp[n][n / 2];
    println!("{}", ans);
}
