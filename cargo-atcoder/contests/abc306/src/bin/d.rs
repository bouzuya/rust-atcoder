use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, i64); n],
    };
    let inf = 1_i64 << 60;
    let mut dp = vec![vec![-inf; 2]; n + 1];
    dp[0][0] = 0_i64;

    for (i, (x, y)) in xy.iter().copied().enumerate() {
        dp[i + 1][0] = dp[i + 1][0].max(dp[i][0]);
        dp[i + 1][1] = dp[i + 1][1].max(dp[i][1]);
        match x {
            0 => {
                dp[i + 1][0] = dp[i + 1][0].max(dp[i][0] + y);
                dp[i + 1][0] = dp[i + 1][0].max(dp[i][1] + y);
            }
            1 => {
                dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + y);
            }
            _ => unreachable!(),
        }
    }

    let ans = dp[n][0].max(dp[n][1]);
    println!("{}", ans);
}
