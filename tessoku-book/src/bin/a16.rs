use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    };
    let inf = 1 << 60;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = (dp[i - 1] + a[i - 1]).min(if i > 1 { dp[i - 2] + b[i - 2] } else { inf })
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
