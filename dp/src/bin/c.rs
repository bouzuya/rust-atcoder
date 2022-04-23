use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [[usize; 3]; n],
    };
    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 1..=n {
        for j in 0..3 {
            for k in 0..3 {
                if j != k {
                    dp[i][j] = dp[i][j].max(dp[i - 1][k] + abc[i - 1][k]);
                }
            }
        }
    }

    let ans = *dp[n].iter().max().unwrap();
    println!("{}", ans);
}
