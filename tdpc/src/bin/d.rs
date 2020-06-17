use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        d: i64,
    };

    let (d2, d3, d5) = {
        let mut d = d; // shadowing

        let mut d2 = 0;
        while d % 2 == 0 {
            d2 += 1;
            d /= 2;
        }

        let mut d3 = 0;
        while d % 3 == 0 {
            d3 += 1;
            d /= 3;
        }

        let mut d5 = 0;
        while d % 5 == 0 {
            d5 += 1;
            d /= 5;
        }

        if d == 1 {
            (d2, d3, d5)
        } else {
            println!("{}", 0_f64);
            return;
        }
    };

    let ans = {
        let mut dp = vec![vec![vec![vec![0_f64; d5 + 1]; d3 + 1]; d2 + 1]; n + 1];
        dp[0][0][0][0] = 1_f64;
        for i in 0..n {
            for j in 0..=d2 {
                for k in 0..=d3 {
                    for l in 0..=d5 {
                        let p = dp[i][j][k][l] * (1_f64 / 6_f64);
                        dp[i + 1][j][k][l] += p;
                        dp[i + 1][min(d2, j + 1)][k][l] += p;
                        dp[i + 1][j][min(d3, k + 1)][l] += p;
                        dp[i + 1][min(d2, j + 2)][k][l] += p;
                        dp[i + 1][j][k][min(d5, l + 1)] += p;
                        dp[i + 1][min(d2, j + 1)][min(d3, k + 1)][l] += p;
                    }
                }
            }
        }
        dp[n][d2][d3][d5]
    };

    println!("{}", ans);
}
