use proconio::input;

fn main() {
    input! {
        n: usize,
        capital_w: usize,
        wv: [(usize, usize); n],
    };
    let inf = 1 << 60;
    let max_v = 100_000;
    let mut dp = vec![inf; max_v + 1];
    dp[0] = 0;
    for (w, v) in wv {
        for i in (0..=max_v).rev() {
            if i + v <= max_v && dp[i] + w <= capital_w {
                dp[i + v] = dp[i + v].min(dp[i] + w);
            }
        }
    }

    let ans = dp
        .iter()
        .enumerate()
        .filter(|(_, &dp_i)| dp_i != inf)
        .map(|(v, _)| v)
        .max()
        .unwrap_or_default();
    println!("{}", ans);
}
