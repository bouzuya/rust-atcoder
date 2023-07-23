use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        x: Chars,
    };
    let mut dp = vec![false; 7];
    dp[0] = true;
    for (s_i, x_i) in s.into_iter().zip(x.into_iter()).rev() {
        let mut next = vec![false; 7];
        let d = (s_i as u8 - b'0') as usize;
        for (r, next_r) in next.iter_mut().enumerate() {
            let d_1 = (10 * r) % 7;
            let d_2 = (10 * r + d) % 7;
            match x_i {
                'T' => {
                    if dp[d_1] || dp[d_2] {
                        *next_r = true;
                    }
                }
                'A' => {
                    if dp[d_1] && dp[d_2] {
                        *next_r = true;
                    }
                }
                _ => unreachable!(),
            }
        }
        dp = next;
    }

    let ans = dp[0];
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
