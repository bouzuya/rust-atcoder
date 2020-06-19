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
        h: [i64; n],
    };
    let inf = 1_000_000_000_000_000_i64;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            chmin!(dp[i + 1], dp[i] + (h[i + 1] - h[i]).abs());
        }
        if i + 2 < n {
            chmin!(dp[i + 2], dp[i] + (h[i + 2] - h[i]).abs());
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
