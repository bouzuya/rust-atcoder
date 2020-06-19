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
        k: usize,
        h: [i64; n],
    };
    let inf = 1_000_000_000_000_000_i64;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n {
        for j in i + 1..=i + k {
            if j < n {
                chmin!(dp[j], dp[i] + (h[j] - h[i]).abs());
            }
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
