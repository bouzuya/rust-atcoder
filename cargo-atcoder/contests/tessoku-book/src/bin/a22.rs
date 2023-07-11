use proconio::{input, marker::Usize1};

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
        b: [Usize1; n - 1],
    };
    let inf = 1_i64 << 60;
    let mut dp = vec![-inf; n];
    dp[0] = 0_i64;
    for (i, (a_i, b_i)) in a.into_iter().zip(b.into_iter()).enumerate() {
        chmax!(dp[a_i], dp[i] + 100_i64);
        chmax!(dp[b_i], dp[i] + 150_i64);
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
