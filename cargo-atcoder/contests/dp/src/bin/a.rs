use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };
    let inf = 1 << 60;
    let mut dp = vec![inf; n + 1];
    dp[1] = 0;
    for j in 2..=n {
        let h_j = h[j - 1];
        for k in 1..=2 {
            if j >= 1 + k {
                let i = j - k;
                let h_i = h[i - 1];
                dp[j] = dp[j].min(dp[i] + (h_j - h_i).abs());
            }
        }
    }
    let ans = dp[n];
    println!("{}", ans);
}
