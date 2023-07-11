use proconio::input;

fn f(x: usize) -> usize {
    let s = format!("{}", x).chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![vec![0; 2]; 2]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        let d_i = s[i] as u8 - '0' as u8;
        for j in 0..2 {
            for k in 0..2 {
                for d in 0..=if j == 1 { 9 } else { d_i } {
                    let ni = i + 1;
                    let nj = if j == 1 || d < d_i { 1 } else { 0 };
                    let nk = if k == 1 || d == 4 || d == 9 { 1 } else { 0 };
                    dp[ni][nj][nk] += dp[i][j][k];
                }
            }
        }
    }
    dp[n][0][1] + dp[n][1][1]
}

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = f(b) - f(a - 1);
    println!("{}", ans);
}
