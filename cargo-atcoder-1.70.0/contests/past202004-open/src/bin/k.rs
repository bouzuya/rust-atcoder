use proconio::{input, marker::Chars};

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [usize; n],
        d: [usize; n],
    };
    let inf = 1 << 60;
    let mut dp = vec![inf; 3_000 + 1];
    dp[0] = 0_usize;
    for ((s_i, c_i), d_i) in s.into_iter().zip(c.into_iter()).zip(d.into_iter()) {
        let mut next = vec![inf; 3_000 + 1];
        for j in 0..=3_000 {
            if s_i == '(' {
                if j < 3_000 {
                    chmin!(next[j + 1], dp[j]);
                }
                chmin!(next[j], dp[j] + d_i);
                if j > 0 {
                    chmin!(next[j - 1], dp[j] + c_i);
                }
            } else {
                if j > 0 {
                    chmin!(next[j - 1], dp[j]);
                }
                chmin!(next[j], dp[j] + d_i);
                if j < 3_000 {
                    chmin!(next[j + 1], dp[j] + c_i);
                }
            }
        }
        dp = next;
    }

    let ans = dp[0];
    println!("{}", ans);
}
