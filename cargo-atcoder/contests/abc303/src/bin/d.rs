use proconio::{input, marker::Chars};

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
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    };
    let n = s.len();
    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; 2]; n + 1];
    dp[0][0] = 0;
    for (i, s_i) in s.iter().copied().enumerate() {
        match s_i {
            'a' => {
                chmin!(dp[i + 1][0], dp[i][0] + x);
                chmin!(dp[i + 1][1], dp[i][0] + z + y);
                chmin!(dp[i + 1][1], dp[i][1] + y);
                chmin!(dp[i + 1][0], dp[i][1] + z + x);
            }
            'A' => {
                chmin!(dp[i + 1][0], dp[i][0] + y);
                chmin!(dp[i + 1][1], dp[i][0] + z + x);
                chmin!(dp[i + 1][1], dp[i][1] + x);
                chmin!(dp[i + 1][0], dp[i][1] + z + y);
            }
            _ => unreachable!(),
        }
    }
    let ans = dp[n][0].min(dp[n][1]);
    println!("{}", ans);
}
