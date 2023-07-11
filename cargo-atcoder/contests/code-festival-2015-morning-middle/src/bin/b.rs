use proconio::input;
use proconio::marker::Chars;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn lcs_len(s: &[char], t: &[char]) -> usize {
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    dp[0][0] = 0;
    for (i, &s_i) in s.iter().enumerate() {
        for (j, &t_j) in t.iter().enumerate() {
            if s_i == t_j {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }
    dp[s.len()][t.len()]
}

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut max_len = 0;
    for m in 0..n {
        let s1 = &s[..m];
        let s2 = &s[m..];
        chmax!(max_len, lcs_len(s1, s2));
    }
    let ans = n - max_len * 2;
    println!("{}", ans);
}
