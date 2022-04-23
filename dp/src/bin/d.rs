use proconio::input;

fn main() {
    input! {
        n: usize,
        capital_w: usize,
        wv: [(usize, usize); n],
    };
    let mut dp = vec![0; capital_w + 1];
    for (w, v) in wv {
        for i in (0..=capital_w).rev() {
            if i + w <= capital_w {
                dp[i + w] = dp[i + w].max(dp[i] + v);
            }
        }
    }

    let ans = *dp.iter().max().unwrap();
    println!("{}", ans);
}
