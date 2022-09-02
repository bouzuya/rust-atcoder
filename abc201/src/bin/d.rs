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
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    // let f = |b: &[Vec<i64>]| {
    //     for i in 0..h {
    //         for j in 0..w {
    //             print!("{:3}", b[i][j]);
    //         }
    //         println!();
    //     }
    //     println!();
    // };

    let mut b = vec![vec![0_i64; w]; h];
    for i in 0..h {
        for j in 0..w {
            b[i][j] = match (a[i][j], (i + j) % 2 == 0) {
                ('+', true) => -1,
                ('+', false) => 1,
                ('-', true) => 1,
                ('-', false) => -1,
                _ => unreachable!(),
            };
        }
    }
    b[0][0] = 0;

    let inf = h as i64 * w as i64 + 1;
    let mut dp = vec![vec![inf; w]; h];
    dp[h - 1][w - 1] = b[h - 1][w - 1];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i != 0 {
                if dp[i - 1][j] == inf {
                    dp[i - 1][j] = dp[i][j] + b[i - 1][j];
                } else if (i + j) % 2 == 0 {
                    chmin!(dp[i - 1][j], dp[i][j] + b[i - 1][j]);
                } else {
                    chmax!(dp[i - 1][j], dp[i][j] + b[i - 1][j]);
                }
            }
            if j != 0 {
                if dp[i][j - 1] == inf {
                    dp[i][j - 1] = dp[i][j] + b[i][j - 1];
                } else if (i + j) % 2 == 0 {
                    chmin!(dp[i][j - 1], dp[i][j] + b[i][j - 1]);
                } else {
                    chmax!(dp[i][j - 1], dp[i][j] + b[i][j - 1]);
                }
            }
        }
    }

    // f(&b);
    // f(&dp);

    let ans = if dp[0][0] == 0 {
        "Draw"
    } else if dp[0][0] > 0 {
        "Takahashi"
    } else {
        "Aoki"
    };
    println!("{}", ans);
}
