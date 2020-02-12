use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        mut s: Bytes,
        nzc: usize
    };
    for i in 0..s.len() {
        s[i] -= b'0';
    }
    let mut dp = vec![vec![vec![0usize; 2]; nzc + 1]; s.len() + 1];
    dp[0][0][0] = 1;
    for i in 0..s.len() {
        let d = s[i];
        for j in 0..=nzc {
            for k in 0..=1 {
                for l in 0..=9 {
                    let ni = i + 1;
                    let nj = if l != 0 { j + 1 } else { j };
                    let nk = if k == 0 && l < d { 1 } else { k };
                    if nj > nzc {
                        continue;
                    }
                    if k == 0 && l > d {
                        continue;
                    }
                    dp[ni][nj][nk] += dp[i][j][k];
                }
            }
        }
    }
    let ans = dp[s.len()][nzc][0] + dp[s.len()][nzc][1];
    println!("{}", ans);
}
