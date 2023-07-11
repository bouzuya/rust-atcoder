// 解説 AC <https://drken1215.hatenablog.com/entry/2019/09/16/014600>
use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (i + 1..n).rev() {
            if s[i] == s[j] {
                dp[i][j] = cmp::max(dp[i][j], dp[i + 1][j + 1] + 1)
            }
        }
    }

    let mut len = 0;
    for i in 0..n {
        for j in i + 1..n {
            len = cmp::max(len, cmp::min(dp[i][j], j - i));
        }
    }

    let ans = len;
    println!("{}", ans);
}
