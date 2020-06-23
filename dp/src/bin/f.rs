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

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for (i, &s_i) in s.iter().enumerate() {
        for (j, &t_i) in t.iter().enumerate() {
            if s_i == t_i {
                chmax!(dp[i + 1][j + 1], dp[i][j] + 1);
            }
            chmax!(dp[i + 1][j + 1], dp[i + 1][j]);
            chmax!(dp[i + 1][j + 1], dp[i][j + 1]);
        }
    }

    let mut p = (s.len(), t.len(), vec![]);
    while p.0 > 0 && p.1 > 0 {
        p = if dp[p.0 - 1][p.1] == dp[p.0][p.1] {
            (p.0 - 1, p.1, p.2)
        } else if dp[p.0][p.1 - 1] == dp[p.0][p.1] {
            (p.0, p.1 - 1, p.2)
        } else {
            p.2.push(s[p.0 - 1]);
            (p.0 - 1, p.1 - 1, p.2)
        }
    }
    println!("{}", p.2.iter().rev().collect::<String>());
}
