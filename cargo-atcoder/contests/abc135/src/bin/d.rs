use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut dp = vec![0_usize; 13];
    dp[0] = 1_usize;
    for s_i in s {
        let mut next = vec![0_usize; 13];
        for (j, dp_j) in dp.iter().copied().enumerate() {
            for k in 0..=9 {
                let ok = match s_i {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        let d_i = s_i.to_digit(10).unwrap() as usize;
                        d_i == k
                    }
                    '?' => true,
                    _ => unreachable!(),
                };
                if ok {
                    next[(j * 10 + k) % dp.len()] += dp_j;
                    next[(j * 10 + k) % dp.len()] %= 1_000_000_007;
                }
            }
        }
        dp = next;
    }
    let ans = dp[5];
    println!("{}", ans);
}
