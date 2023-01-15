use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut dp = vec![1_i64 << 60; n];
    dp[0] = 0_i64;
    for i in 0..n {
        if i + 1 < n {
            chmin!(dp[i + 1], dp[i] + (a[i] - a[i + 1]).abs());
        }
        if i + 2 < n {
            chmin!(dp[i + 2], dp[i] + (a[i] - a[i + 2]).abs());
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
