use proconio::{input, marker::Chars};

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
    }
    let n = s.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for len in 3..=n {
        for l in 0..n {
            let r = l + len;
            if r > n {
                break;
            }
            if s[l] != 'i' || s[r - 1] != 'i' {
                continue;
            }
            for m in l + 1..r - 1 {
                if s[m] != 'w' {
                    continue;
                }
                let count_l = dp[l + 1][m];
                if count_l * 3 != m - l - 1 {
                    continue;
                }
                let count_r = dp[m + 1][r - 1];
                if count_r * 3 != r - 1 - m - 1 {
                    continue;
                }
                chmax!(dp[l][r], (r - l) / 3);
            }
        }
        for l in 0..n {
            let r = l + len;
            if r > n {
                break;
            }
            for m in l + 1..r {
                chmax!(dp[l][r], dp[l][m] + dp[m][r]);
            }
        }
    }
    let ans = dp[0][n];
    println!("{}", ans);
}
