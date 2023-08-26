use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(usize, usize, usize); n],
    };
    let sum_z = xyz.iter().copied().map(|(_, _, z)| z).sum::<usize>();
    let max_z = sum_z;
    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; max_z + 1]; n + 1];
    dp[0][0] = 0;
    for (i, (x, y, z)) in xyz.iter().copied().enumerate() {
        for j in 0..=max_z {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            let r = if x < y { (y - x) / 2 + 1 } else { 0 };
            let nj = (j + z).min(max_z);
            dp[i + 1][nj] = dp[i + 1][nj].min(dp[i][j] + r);
        }
    }
    println!(
        "{}",
        dp[n]
            .iter()
            .enumerate()
            .filter(|&(j, _)| j > sum_z / 2)
            .map(|(_, i)| i)
            .min()
            .unwrap()
    );
}
