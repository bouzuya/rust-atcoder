use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    // let mut dp = vec![vec![0; k + 1]; n + 1];
    // dp[0][0] = 1;
    // for i in 0..n {
    //     for j in 0..=k {
    //         for l in 0..=a[i] {
    //             if j + l <= k {
    //                 dp[i + 1][j + l] += dp[i][j];
    //                 dp[i + 1][j + l] %= 1_000_000_007;
    //             }
    //         }
    //     }
    // }
    // let ans = dp[n][k];
    // println!("{}", ans);

    // let mut dp = vec![vec![0; k + 1]; n + 1];
    // dp[0][0] = 1;
    // for i in 0..n {
    //     for j in 0..=k {
    //         for l in 0..=a[i] {
    //             if j >= l {
    //                 dp[i + 1][j] += dp[i][j - l];
    //                 dp[i + 1][j] %= 1_000_000_007;
    //             }
    //         }
    //     }
    // }
    // let ans = dp[n][k];
    // println!("{}", ans);

    // let mut dp = vec![0; k + 1];
    // dp[0] = 1;
    // for a_i in a.iter().copied() {
    //     let mut next = vec![0; k + 1];
    //     for j in 0..=k {
    //         for l in 0..=a_i {
    //             if j >= l {
    //                 next[j] += dp[j - l];
    //                 next[j] %= 1_000_000_007;
    //             }
    //         }
    //     }
    //     dp = next;
    // }
    // let ans = dp[k];
    // println!("{}", ans);

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for a_i in a.iter().copied() {
        let cumsum = std::iter::once(0)
            .chain(dp.iter().scan(0, |acc, &i| {
                *acc += i;
                Some(*acc)
            }))
            .collect::<Vec<usize>>();
        let mut next = vec![0; k + 1];
        for j in 0..=k {
            next[j] = cumsum[j + 1] - cumsum[j.saturating_sub(a_i)];
            next[j] %= 1_000_000_007;
        }
        dp = next;
    }
    let ans = dp[k];
    println!("{}", ans);
}
