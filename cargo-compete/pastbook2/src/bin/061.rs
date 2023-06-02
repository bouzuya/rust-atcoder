use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let p = 1_000_000_007;
    let t = "atcoder".chars().collect::<Vec<char>>();
    let mut dp = vec![vec![0; t.len() + 1]; n + 1];
    dp[0][0] = 1_usize;
    for (i, s_i) in s.iter().copied().enumerate() {
        for (j, t_j) in t.iter().copied().enumerate() {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= p;
            if s_i == t_j {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= p;
            }
        }
        dp[i + 1][t.len()] += dp[i][t.len()];
        dp[i + 1][t.len()] %= p;
    }
    let ans = dp[n][t.len()];
    println!("{}", ans);
}
