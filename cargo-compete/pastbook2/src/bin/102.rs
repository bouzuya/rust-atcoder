use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }
    let modp = 1_000_000_007_usize;
    let mut dp = vec![0_usize; 1 << n];
    dp[0] = 1_usize;
    for set in 1_usize..1 << n {
        let i = (set.count_ones() - 1) as usize;
        for j in 0..n {
            if a[i][j] == 1 && (set & (1 << j)) != 0 {
                dp[set] += dp[set ^ (1 << j)];
                dp[set] %= modp;
            }
        }
    }
    let ans = dp[(1 << n) - 1];
    println!("{}", ans);
}
