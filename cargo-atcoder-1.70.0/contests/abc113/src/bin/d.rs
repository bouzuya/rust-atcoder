use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: Usize1,
    };
    if w == 1 {
        println!("1");
        return;
    }

    let mut dp = vec![vec![0; w]; h + 1];
    dp[0][0] = 1_usize;
    for i in 0..h {
        for bits in 0..(1 << (w - 1)) {
            let js = (0..w - 1)
                .map(|i| ((bits >> i) & 1) == 1)
                .collect::<Vec<bool>>();
            let mut ok = true;
            let mut p = false;
            for (_, b) in js.iter().copied().enumerate() {
                if p && b {
                    ok = false;
                    break;
                }
                p = b;
            }
            if !ok {
                continue;
            }
            for j in 0..w {
                // 0 1 2 3
                // |-| |-|
                // t f t
                if j == 0 {
                    if js[j] {
                        dp[i + 1][j] += dp[i][j + 1];
                        dp[i + 1][j] %= 1_000_000_007;
                    } else {
                        dp[i + 1][j] += dp[i][j];
                        dp[i + 1][j] %= 1_000_000_007;
                    }
                } else if j == w - 1 {
                    if js[j - 1] {
                        dp[i + 1][j] += dp[i][j - 1];
                        dp[i + 1][j] %= 1_000_000_007;
                    } else {
                        dp[i + 1][j] += dp[i][j];
                        dp[i + 1][j] %= 1_000_000_007;
                    }
                } else {
                    if js[j - 1] {
                        dp[i + 1][j] += dp[i][j - 1];
                        dp[i + 1][j] %= 1_000_000_007;
                    }
                    if js[j] {
                        dp[i + 1][j] += dp[i][j + 1];
                        dp[i + 1][j] %= 1_000_000_007;
                    }
                    if !js[j - 1] && !js[j] {
                        dp[i + 1][j] += dp[i][j];
                        dp[i + 1][j] %= 1_000_000_007;
                    }
                }
            }
        }
    }
    let ans = dp[h][k];
    println!("{}", ans);
}
