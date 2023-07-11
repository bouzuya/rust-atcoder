use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut dp = vec![vec![0; n]; n];
    for len in 1..=n {
        for l in 0..=n - len {
            // [l, r]
            let r = l + len - 1;

            dp[l][r] = if l == r {
                1
            } else if l + 1 == r {
                if s[l] == s[r] {
                    2
                } else {
                    1
                }
            } else {
                (dp[l + 1][r - 1] + if s[l] == s[r] { 2 } else { 0 })
                    .max(dp[l][r - 1])
                    .max(dp[l + 1][r])
            };
        }
    }
    let ans = dp[0][n - 1];
    println!("{}", ans);
}
