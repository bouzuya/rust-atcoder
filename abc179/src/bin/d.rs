use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        lr: [(usize, usize); k],
    };
    let modp = 998244353;
    let mut dp = vec![0; n + 1];
    let mut cp = vec![0; n + 1];
    dp[1] = 1;
    cp[1] = 1;
    for i in 2..=n {
        for &(l_j, r_j) in lr.iter() {
            let i_l = if i <= r_j { 1 } else { i - r_j };
            let i_r = if i <= l_j { 0 } else { i - l_j };
            dp[i] += cp[i_r] + (if cp[i_r] < cp[i_l - 1] { modp } else { 0 } - cp[i_l - 1]);
            dp[i] %= modp;
        }
        cp[i] += cp[i - 1] + dp[i];
        cp[i] %= modp;
    }
    let ans = dp[n];
    println!("{}", ans);
}
