use ac_library::ModInt1000000007 as ModInt;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let chars = "atcoder".chars().collect::<Vec<char>>();
    let mut dp = vec![vec![ModInt::new(0); chars.len() + 1]; n + 1];
    dp[0][0] = ModInt::new(1);
    for (i, s_i) in s.into_iter().enumerate() {
        for j in 0..=chars.len() {
            dp[i + 1][j] = dp[i + 1][j] + dp[i][j];
            if j >= chars.len() {
                continue;
            }
            let c = chars[j];
            if c == s_i {
                if i + 1 > n || j >= chars.len() {
                    continue;
                }
                dp[i + 1][j + 1] = dp[i + 1][j + 1] + dp[i][j];
            }
        }
    }

    let ans = dp[n][chars.len()];
    println!("{}", ans);
}
