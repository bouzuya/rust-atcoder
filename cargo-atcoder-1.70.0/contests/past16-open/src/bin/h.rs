use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![None; 2]; n + 1];
    dp[0][0] = Some(0_usize);
    for a_i in a.iter().copied() {
        let mut next = vec![vec![None; 2]; n + 1];
        for j in 0..=n {
            if let Some(v) = dp[j][0] {
                next[j][0] = next[j][0].max(Some(v + a_i));
                if j + 1 <= n {
                    next[j + 1][1] = next[j + 1][1].max(Some(v));
                }
            }
            if let Some(v) = dp[j][1] {
                next[j][0] = next[j][0].max(Some(v + a_i));
            }
        }
        dp = next;
    }

    let mut ans = 0_usize;
    for i in m..=n {
        ans = ans.max(dp[i][0].max(dp[i][1]).unwrap_or(0));
    }
    println!("{}", ans);
}
