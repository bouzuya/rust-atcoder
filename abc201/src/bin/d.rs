use std::cmp::{max, min};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };
    if h == 1 && w == 1 {
        println!("Draw");
        return;
    }
    let mut b = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            b[i][j] = if (i + j) % 2 == 0 {
                match a[i][j] {
                    '+' => -1,
                    '-' => 1,
                    _ => unreachable!(),
                }
            } else {
                match a[i][j] {
                    '+' => 1,
                    '-' => -1,
                    _ => unreachable!(),
                }
            };
        }
    }

    let inf = 1_000_000_000;
    let mut dp = vec![vec![inf; w + 1]; h + 1];
    dp[h - 1][w - 1] = b[h - 1][w - 1];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 && j == w - 1 {
                continue;
            }
            dp[i][j] = if (i + j) % 2 == 0 {
                max(
                    if dp[i][j + 1] == inf {
                        -inf
                    } else {
                        dp[i][j + 1]
                    },
                    if dp[i + 1][j] == inf {
                        -inf
                    } else {
                        dp[i + 1][j]
                    },
                )
            } else {
                min(dp[i][j + 1], dp[i + 1][j])
            } + if i == 0 && j == 0 { 0 } else { b[i][j] };
        }
    }

    let ans = if dp[0][0] == 0 {
        "Draw"
    } else if dp[0][0] > 0 {
        "Takahashi"
    } else {
        "Aoki"
    };
    println!("{}", ans);
}
