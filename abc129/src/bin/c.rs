use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };
    let p = 1_000_000_007_usize;
    let mut j = 0_usize;
    let mut dp = vec![0_usize; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        if j < m && i == a[j] {
            j += 1;
            continue;
        }
        dp[i] += dp[i - 1];
        dp[i] %= p;
        if i >= 2 {
            dp[i] += dp[i - 2];
            dp[i] %= p;
        }
    }
    let ans = dp[n];
    println!("{}", ans);
}
