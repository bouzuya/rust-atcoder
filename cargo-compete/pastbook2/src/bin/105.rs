use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
    }

    let mut c = vec![vec![0_usize; n + 1]; m];
    for i in 0..m {
        for (j, a_j) in a.iter().copied().enumerate() {
            c[i][j + 1] = c[i][j] + if a_j == i { 1 } else { 0 };
        }
    }

    let inf = 1_usize << 60;
    let mut dp = vec![inf; 1 << m];
    dp[0] = 0_usize;
    for bits in 1_usize..1 << m {
        let right = (0..m)
            .filter(|i| (bits & (1_usize << i)) != 0)
            .map(|i| c[i][n])
            .sum::<usize>();

        for i in 0..m {
            if (bits & (1 << i)) == 0 {
                continue;
            }

            let left = right - c[i][n];
            let count = right - left - (c[i][right] - c[i][left]);
            dp[bits] = dp[bits].min(dp[bits ^ (1 << i)] + count);
        }
    }
    let ans = dp[(1 << m) - 1];
    println!("{}", ans);
}
