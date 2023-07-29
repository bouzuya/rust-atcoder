use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let p = 998_244_353_usize;
    let mut dp = vec![0_usize; 3000 + 1];
    dp[0] = 1_usize;
    for s_i in s.iter().copied() {
        let mut next = vec![0_usize; 3000 + 1];
        for j in 0..=3000 {
            match s_i {
                '(' => {
                    if j + 1 <= 3000 {
                        next[j + 1] += dp[j];
                        next[j + 1] %= p;
                    }
                }
                ')' => {
                    if j > 0 {
                        next[j - 1] += dp[j];
                        next[j - 1] %= p;
                    }
                }
                '?' => {
                    if j + 1 <= 3000 {
                        next[j + 1] += dp[j];
                        next[j + 1] %= p;
                    }
                    if j > 0 {
                        next[j - 1] += dp[j];
                        next[j - 1] %= p;
                    }
                }
                _ => unreachable!(),
            }
        }
        dp = next;
    }

    let ans = dp[0];
    println!("{}", ans);
}
