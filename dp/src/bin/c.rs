use proconio::input;

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
        abc: [(i64, i64, i64); n],
    };
    // abc[i]: i - 1 日目の選択肢
    // dp[i][j]: i 日目までで最後に j を選んでいたときに得る幸福度の総和の最大値
    // j は 0..3 で 0: A, 1: B, 2: C
    let mut dp = vec![vec![0; 3]; n + 1];
    for (i, &(a_i, b_i, c_i)) in abc.iter().enumerate() {
        chmax!(dp[i + 1][0], std::cmp::max(dp[i][1] + a_i, dp[i][2] + a_i));
        chmax!(dp[i + 1][1], std::cmp::max(dp[i][0] + b_i, dp[i][2] + b_i));
        chmax!(dp[i + 1][2], std::cmp::max(dp[i][0] + c_i, dp[i][1] + c_i));
    }
    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
