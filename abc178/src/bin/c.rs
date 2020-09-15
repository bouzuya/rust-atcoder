use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut dp = vec![vec![vec![0_i64; 2]; 2]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..2 {
            for k in 0..2 {
                for d in 0..=9 {
                    let j_next = if j == 1 || d == 0 { 1 } else { 0 };
                    let k_next = if k == 1 || d == 9 { 1 } else { 0 };
                    dp[i + 1][j_next][k_next] += dp[i][j][k];
                    dp[i + 1][j_next][k_next] %= 1_000_000_007;
                }
            }
        }
    }
    let ans = dp[n][1][1];
    println!("{}", ans);
}
