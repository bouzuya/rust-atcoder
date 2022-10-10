use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]).max(if s[i - 1] == t[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                0
            });
        }
    }
    let ans = dp[n][m];
    println!("{}", ans);
}
