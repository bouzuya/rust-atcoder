use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }

    let mut cost = vec![vec![0; m + 1]; n];
    for i in 0..n {
        let (mut min, mut max) = (1 << 60, 0);
        for j in i..(i + m).min(n) {
            let len = j - i + 1;
            min = min.min(a[j]);
            max = max.max(a[j]);
            cost[i][len] = k + (max - min) * len;
        }
    }

    let inf = 1_usize << 60;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0_usize;
    for i in 1..=n {
        for j in i.saturating_sub(m)..i {
            dp[i] = dp[i].min(dp[j] + cost[j][i - j]);
        }
    }
    let ans = dp[n];
    println!("{}", ans);
}
