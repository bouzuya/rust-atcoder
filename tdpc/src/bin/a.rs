use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let mut dp = vec![vec![false; 100 * 100 + 1]; n + 1];
    dp[0][0] = true;
    for (i, &p_i) in p.iter().enumerate() {
        for j in 0..=100 * 100 {
            dp[i + 1][j] |= dp[i][j];
            if j + p_i <= 100 * 100 {
                dp[i + 1][j + p_i] |= dp[i][j];
            }
        }
    }
    let ans = dp[n].iter().filter(|&&x| x).count();
    println!("{}", ans);
}
