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
        a: [usize; n],
    };
    let inf = 1_usize << 60;

    let mut dp = vec![vec![inf; 2]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = a[n - 1];
    for i in 0..n {
        chmin!(dp[i + 1][1], dp[i][0] + a[i]);
        chmin!(dp[i + 1][0], dp[i][1]);
        chmin!(dp[i + 1][1], dp[i][1] + a[i]);
    }
    let ans1 = *dp[n].iter().min().unwrap();

    let mut dp = vec![vec![inf; 2]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;
    for i in 0..n {
        chmin!(dp[i + 1][1], dp[i][0] + a[i]);
        chmin!(dp[i + 1][0], dp[i][1]);
        chmin!(dp[i + 1][1], dp[i][1] + a[i]);
    }
    let ans2 = dp[n][1];

    let ans = ans1.min(ans2);
    println!("{}", ans);
}
