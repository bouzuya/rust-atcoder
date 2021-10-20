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

    let inf = 1_000_000_001;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n {
        for j in i + 1..=i + 2 {
            if j < n {
                chmin!(dp[j], dp[i] + (h[i] - h[j]).abs());
            }
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
