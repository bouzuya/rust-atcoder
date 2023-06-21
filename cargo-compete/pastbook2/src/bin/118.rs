use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let modp = 1_000_000_007_usize;
    let mut dp = vec![vec![0_usize; w]; h];
    let mut sum_h = vec![vec![0_usize; w + 1]; h + 1];
    let mut sum_v = vec![vec![0_usize; w + 1]; h + 1];
    let mut sum_d = vec![vec![0_usize; w + 1]; h + 1];
    dp[0][0] = 1_usize;
    sum_h[0][1] = 1_usize;
    sum_v[1][0] = 1_usize;
    sum_d[1][1] = 1_usize;
    for i in 0..h {
        for j in 0..w {
            match s[i][j] {
                '#' => {
                    sum_h[i][j + 1] = 0;
                    sum_v[i + 1][j] = 0;
                    sum_d[i + 1][j + 1] = 0;
                }
                '.' => {
                    dp[i][j] += sum_h[i][j];
                    dp[i][j] += sum_v[i][j];
                    dp[i][j] += sum_d[i][j];
                    dp[i][j] %= modp;

                    sum_h[i][j + 1] = sum_h[i][j] + dp[i][j];
                    sum_h[i][j + 1] %= modp;
                    sum_v[i + 1][j] = sum_v[i][j] + dp[i][j];
                    sum_v[i + 1][j] %= modp;
                    sum_d[i + 1][j + 1] = sum_d[i][j] + dp[i][j];
                    sum_d[i + 1][j + 1] %= modp;
                }
                _ => unreachable!(),
            }
        }
    }
    let ans = dp[h - 1][w - 1];
    println!("{}", ans);
}
