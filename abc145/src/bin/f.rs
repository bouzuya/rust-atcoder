use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        hv: [usize; n],
    };
    let m = n - k;
    // m 個残してΣmax(0, h[i+1] - h[i]) を最小にする
    let inf = 1_000_000_000_000_000_000_usize;
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = (0..i + 1)
                .map(|k| dp[k][j] + hv[i].saturating_sub(if k >= 1 { hv[k - 1] } else { 0 }))
                .min()
                .unwrap();
        }
    }
    let ans = (0..=n).map(|i| dp[i][m]).min().unwrap();
    println!("{}", ans);
}
