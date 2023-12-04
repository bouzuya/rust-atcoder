use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        pqlr: [(i64, Usize1, usize, usize); m],
    };
    let mut plr = vec![vec![]; n + 1];
    for (p, q, l, r) in pqlr {
        plr[q].push((p, l, r));
    }

    let inf = 1_i64 << 60;
    let mut ans = 0_i64;
    for k in 0..3 {
        let mut dp = vec![vec![-inf, -inf, -inf]; n + 1];
        dp[0][0] = 0_i64;
        for (p, l, r) in plr[0].iter().copied() {
            if l == 0 && (l + r) % 3 == k {
                dp[0][0] += p;
            }
        }
        for (i, plr_i) in plr.iter().enumerate().skip(1) {
            for j in 0..3 {
                dp[i][j] = (dp[i - 1][(j + 3 - 1) % 3] + a[i - 1]).max(dp[i - 1][j]);
                for (p, l, r) in plr_i.iter().copied() {
                    if l == j && (l + r) % 3 == k {
                        dp[i][j] += p;
                    }
                }
            }
        }
        ans = ans.max(dp[n][k]);
    }
    println!("{}", ans);
}
