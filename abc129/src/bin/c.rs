use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };
    let mut ok = vec![true; n + 1];
    for &a_i in a.iter() {
        ok[a_i] = false;
    }
    let p = 1_000_000_007_i64;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        if i + 1 <= n && ok[i + 1] {
            dp[i + 1] += dp[i];
            dp[i + 1] %= p;
        }
        if i + 2 <= n && ok[i + 2] {
            dp[i + 2] += dp[i];
            dp[i + 2] %= p;
        }
    }
    let ans = dp[n];
    println!("{}", ans);
}
