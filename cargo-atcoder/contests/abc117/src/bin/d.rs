use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    };

    let max_digits = 1_000_000_000_000_usize.next_power_of_two().trailing_zeros() as usize;
    // dp[i][j]: 上から i ビット目まででの f の最大値
    // j = 0 なら K 未満が確定していない j = 1 なら K 未満が確定している
    let mut dp = vec![vec![None; 2]; max_digits + 1];
    dp[0][0] = Some(0);
    for i in 0..max_digits {
        let b = 1_u64 << (max_digits - i - 1);
        let c = a.iter().filter(|&&a_i| (a_i & b) == b).count() as u64;

        let x0 = c * b;
        let x1 = (a.len() as u64 - c) * b;

        if let Some(p1) = dp[i][1] {
            dp[i + 1][1] = Some(std::cmp::max(p1, p1 + std::cmp::max(x0, x1)));
        }

        if let Some(p0) = dp[i][0] {
            if (k & b) == b {
                dp[i + 1][1] = std::cmp::max(dp[i + 1][1], Some(p0 + x0));
                dp[i + 1][0] = Some(std::cmp::max(p0, p0 + x1));
            } else {
                dp[i + 1][0] = Some(std::cmp::max(p0, p0 + x0));
            }
        }
    }

    let ans = dp[max_digits]
        .iter()
        .map(|&x| x.unwrap_or(0))
        .max()
        .unwrap();
    println!("{}", ans);
}
