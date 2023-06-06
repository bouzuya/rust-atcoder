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
        d: [usize; n]
    }
    let inf = 1_usize << 60;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0_usize;
    for (i, s_i) in s.iter().copied().enumerate() {
        let mut next = vec![inf; n + 1];
        for j in 0..=n {
            match s_i {
                '(' => {
                    if j + 1 <= n {
                        chmin!(next[j + 1], dp[j]);
                    }
                    if j > 0 {
                        chmin!(next[j - 1], dp[j] + c[i]);
                    }
                    chmin!(next[j], dp[j] + d[i]);
                }
                ')' => {
                    if j > 0 {
                        chmin!(next[j - 1], dp[j]);
                    }
                    if j + 1 <= n {
                        chmin!(next[j + 1], dp[j] + c[i]);
                    }
                    chmin!(next[j], dp[j] + d[i]);
                }
                _ => unreachable!(),
            }
        }
        dp = next;
    }

    let ans = dp[0];
    println!("{}", ans);
}
