use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let mut dp = vec![vec![0_usize; w]; h];
    dp[0][0] = 1_usize;
    for i in 0..h {
        for j in 0..w {
            if j + 1 < w && c[i][j + 1] == '.' {
                dp[i][j + 1] += dp[i][j];
            }
            if i + 1 < h && c[i + 1][j] == '.' {
                dp[i + 1][j] += dp[i][j];
            }
        }
    }

    let ans = dp[h - 1][w - 1];
    println!("{}", ans);
}
