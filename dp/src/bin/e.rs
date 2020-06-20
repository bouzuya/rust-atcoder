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
        w: usize,
        wv: [(usize, usize); n],
    };
    let max_w = 1_000_000_000 * n;
    let max_v = 1_000 * n;
    let mut dp = vec![vec![max_w + 1; max_v + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &(w_i, v_i)) in wv.iter().enumerate() {
        for j in 0..max_v {
            chmin!(dp[i + 1][j], dp[i][j]);
            if j + v_i <= max_v {
                chmin!(dp[i + 1][j + v_i], dp[i][j] + w_i);
            }
        }
    }
    let ans = dp[n]
        .iter()
        .enumerate()
        .filter(|&(_, &w_i)| w_i <= w)
        .map(|(i, _)| i)
        .max()
        .unwrap();
    println!("{}", ans);
}
