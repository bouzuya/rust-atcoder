use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        mut xc: [(i64, i64); n],
    };
    xc.sort_by_key(|&(x, c)| (c, x));
    let mut m = vec![];
    m.push((xc[0].0, xc[0].0));
    let mut p = xc[0];
    for &(x_i, c_i) in xc.iter().skip(1) {
        if p.1 != c_i {
            m.push((x_i, x_i));
        }
        let last = m.last_mut().unwrap();
        last.0 = min(last.0, x_i);
        last.1 = max(last.1, x_i);
        p = (x_i, c_i);
    }

    let inf = 1_000_000_000_000_000_i64;
    // dp[i][j] := m の i 番目までみて、最後の選択が j = 0 なら最小側 j = 1 なら最大側のときの、最小の距離
    let mut dp = vec![vec![inf; 2]; m.len() + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;
    for (i, &(min_x, max_x)) in m.iter().enumerate() {
        let d = (max_x - min_x).abs();
        let prev_min_x = if i == 0 { 0 } else { m[i - 1].0 };
        let prev_max_x = if i == 0 { 0 } else { m[i - 1].1 };
        dp[i + 1][0] = min(
            dp[i][0] + (prev_min_x - max_x).abs(),
            dp[i][1] + (prev_max_x - max_x).abs(),
        ) + d;
        dp[i + 1][1] = min(
            dp[i][0] + (prev_min_x - min_x).abs(),
            dp[i][1] + (prev_max_x - min_x).abs(),
        ) + d;
    }

    let ans = min(
        dp[m.len()][0] + (m[m.len() - 1].0 - 0).abs(),
        dp[m.len()][1] + (m[m.len() - 1].1 - 0).abs(),
    );
    println!("{}", ans);
}
