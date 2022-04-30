use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    };
    let mut dp = vec![0_f64; n + 1];
    dp[0] = 1_f64 - p[0];
    dp[1] = p[0];
    for p_i in p.iter().copied().skip(1) {
        let mut next = vec![0_f64; n + 1];
        for j in 0..=n {
            next[j] += dp[j] * (1_f64 - p_i);
            if j + 1 <= n {
                next[j + 1] += dp[j] * p_i;
            }
        }
        dp = next;
    }
    let ans = dp
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i > n / 2)
        .map(|(_, p_i)| p_i)
        .sum::<f64>();
    println!("{}", ans);
}
