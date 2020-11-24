use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    // dp[x][y][z]: 袋に金貨 x 枚 銀貨 y 枚 銅貨 z 枚が入っているときの残りの操作回数の期待値
    let mut dp = vec![vec![vec![0_f64; 100 + 1]; 100 + 1]; 100 + 1];
    for x in (a..100).rev() {
        for y in (b..100).rev() {
            for z in (c..100).rev() {
                let p_x = x as f64 / (x + y + z) as f64 * (dp[x + 1][y][z] + 1_f64);
                let p_y = y as f64 / (x + y + z) as f64 * (dp[x][y + 1][z] + 1_f64);
                let p_z = z as f64 / (x + y + z) as f64 * (dp[x][y][z + 1] + 1_f64);
                dp[x][y][z] += p_x + p_y + p_z;
            }
        }
    }

    let ans = dp[a][b][c];
    println!("{:.10}", ans);
}
