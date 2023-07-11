// 解説 AC <https://twitter.com/e869120/status/1381064464049401856>
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
        dcs: [(usize, usize, u64); n],
    };
    let max_d = 5_000;
    let mut dcs = dcs
        .iter()
        .filter(|&(d_i, c_i, _)| c_i <= d_i)
        .cloned()
        .collect::<Vec<(usize, usize, u64)>>();
    dcs.sort_by_key(|&(d_i, _, _)| d_i);

    let mut dp = vec![vec![0_u64; 2]; max_d + 1];
    for (d_i, c_i, s_i) in dcs.iter() {
        for d in (0..max_d).rev() {
            if d <= d_i - c_i {
                chmax!(dp[d + c_i][0], dp[d][0] + s_i);
            }
        }
        for d in 0..max_d {
            chmax!(dp[d + 1][0], dp[d][0]);
        }
    }
    let ans = dp[max_d][0];
    println!("{}", ans);
}
