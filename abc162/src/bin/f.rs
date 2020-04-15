use proconio::input;

fn dpj(i: usize, j_count: usize) -> Option<usize> {
    let j0_count = (i / 2 + 1) - 1;
    if (j0_count..j0_count + 3).contains(&j_count) {
        Some(j_count - j0_count)
    } else {
        None
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let inf: i64 = 1_000_000_000_000_000_005_i64;
    let mut dp = vec![vec![-inf; 3]; n];
    for i in 0..n {
        if let Some(j) = dpj(i, 1) {
            dp[i][j] = a[i];
        }
        let j0_count = (i / 2 + 1) - 1;
        for j_count in j0_count..j0_count + 3 {
            if let Some(j) = dpj(i, j_count) {
                for i2 in i + 2..=std::cmp::min(n - 1, i + 4) {
                    if let Some(j2) = dpj(i2, j_count + 1) {
                        dp[i2][j2] = std::cmp::max(dp[i2][j2], dp[i][j] + a[i2]);
                    }
                }
            }
        }
    }

    let ans = if n % 2 == 0 {
        std::cmp::max(dp[n - 1][1], dp[n - 2][0])
    } else {
        dp[n - 1][0]
    };
    println!("{}", ans);
}
