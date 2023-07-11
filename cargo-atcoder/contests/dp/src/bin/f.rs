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
        let s_i = s[i - 1];
        for j in 1..=m {
            let t_j = t[j - 1];
            dp[i][j] = if s_i == t_j {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }

    let mut ans = vec![];
    let mut i = n;
    let mut j = m;
    while i > 0 && j > 0 {
        if s[i - 1] == t[j - 1] {
            ans.push(s[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            unreachable!();
        }
    }
    ans.reverse();

    println!("{}", ans.iter().collect::<String>());
}
