use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: Chars,
    };
    let p = 1_000_000_007_usize;
    let n = l.len();
    let mut dp = vec![vec![0_usize; 2]; n + 1];
    dp[0][0] = 1;
    for (i, &l_i) in l.iter().enumerate() {
        match l_i {
            '0' => {
                // (0, 0)
                dp[i + 1][0] += dp[i][0];
                dp[i + 1][0] %= p;
                dp[i + 1][1] += dp[i][1];
                dp[i + 1][1] %= p;
                // (0, 1) | (1, 0)
                dp[i + 1][1] += dp[i][1] * 2;
                dp[i + 1][1] %= p;
            }
            '1' => {
                // (0, 0)
                dp[i + 1][1] += dp[i][0];
                dp[i + 1][1] %= p;
                dp[i + 1][1] += dp[i][1];
                dp[i + 1][1] %= p;
                // (0, 1) | (1, 0)
                dp[i + 1][0] += dp[i][0] * 2;
                dp[i + 1][0] %= p;
                dp[i + 1][1] += dp[i][1] * 2;
                dp[i + 1][1] %= p;
            }
            _ => unreachable!(),
        }
    }
    let ans = dp[n].iter().sum::<usize>() % p;
    println!("{}", ans);
}
