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
        mut ab: [(usize, usize); n],
    };
    ab.sort_by_key(|&(a_i, b_i)| (b_i, a_i));

    // dp[i][j] := i 番目までで j 個のファーストアクセプタンスのときの最短時間
    let inf = 1_000_000_000;
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        for j in 0..n {
            chmin!(dp[i + 1][j], dp[i][j]);
            if dp[i][j] + a_i <= b_i {
                chmin!(dp[i + 1][j + 1], dp[i][j + 1]);
                chmin!(dp[i + 1][j + 1], dp[i][j] + a_i);
            }
        }
    }
    let mut ans = 0;
    for (i, &v) in dp[n].iter().enumerate() {
        if v != inf {
            ans = i;
        }
    }
    println!("{}", ans);
}
