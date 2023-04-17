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
    }
    let inf = 1_i64 << 60;
    let mut dp = vec![inf; n];
    dp[0] = 0_i64;
    for i in 1..n {
        for j in i.saturating_sub(2)..i {
            chmin!(dp[i], dp[j] + (h[i] - h[j]).abs());
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
