use proconio::input;

fn main() {
    input! {
        n :usize,
        av: [i64; n],
    };
    let inf = 1_000_000_000_000_000_i64;
    let m = if n % 2 == 0 { 1 } else { 2 };
    // dp[i][j] = x
    // i (0<=i<=n) までで j (0<=j<=m) 個を飛ばしたときの最大値が x
    let mut dp = vec![vec![-inf; m + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &a) in av.iter().enumerate() {
        for j in 0..=m {
            // 飛ばす場合
            if j + 1 <= m {
                dp[i + 1][j + 1] = std::cmp::max(dp[i + 1][j + 1], dp[i][j]);
            }
            // 飛ばさない場合
            dp[i + 1][j] = std::cmp::max(
                dp[i + 1][j],
                dp[i][j] + if (i + j) % 2 == 0 { a } else { 0 },
            );
        }
    }
    let ans = dp[n][m];
    println!("{}", ans);
}
