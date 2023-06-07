use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [[usize; n + 1]; n + 1],
    }

    let inf = 1_usize << 60;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0_usize;
    for i in 0..n {
        for j in 0..=i {
            dp[i + 1] = dp[i + 1].min(dp[j] + c[j][i + 1]);
        }
    }
    let ans = dp[n];
    println!("{}", ans);
}
