use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let n = s.len();
    let m = t.len();
    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        dp[i][0] = i;
    }
    for j in 1..=m {
        dp[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + if s[i - 1] == t[j - 1] { 0 } else { 1 });
        }
    }
    let ans = dp[n][m];
    println!("{}", ans);
}
