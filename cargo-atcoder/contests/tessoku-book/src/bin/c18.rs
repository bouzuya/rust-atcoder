use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 2 * n],
    };
    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; 2 * n]; 2 * n];
    let d = |a: usize, b: usize| -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    };
    for l in 0..2 * n {
        let r = l + 1;
        if r >= 2 * n {
            continue;
        }
        dp[l][r] = d(a[l], a[r]);
    }
    for len in (3..=2 * n).step_by(2) {
        for l in 0..2 * n {
            let r = l + len;
            if r >= 2 * n {
                continue;
            }
            let mut min = dp[l + 1][r - 1] + d(a[l], a[r]);
            for m in l..r {
                min = min.min(dp[l][m] + dp[m + 1][r]);
            }
            dp[l][r] = min;
        }
    }
    let ans = dp[0][2 * n - 1];
    println!("{}", ans);
}
