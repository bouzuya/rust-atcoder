use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n],
    };

    let dist = {
        let mut d1 = vec![vec![0_i64; n]; n];
        for (i, (x_i, y_i)) in xy.iter().copied().enumerate() {
            for (j, (x_j, y_j)) in xy.iter().copied().enumerate() {
                d1[i][j] = (x_i - x_j).pow(2) + (y_i - y_j).pow(2);
            }
        }
        let mut d2 = vec![0_i64; 1 << n];
        for s in 1..1 << n {
            for i in 0..n {
                for j in 0..i {
                    if ((s >> i) & 1) == 1 && ((s >> j) & 1) == 1 {
                        d2[s] = d2[s].max(d1[i][j]);
                    }
                }
            }
        }
        d2
    };

    let inf = std::i64::MAX;
    let mut dp = vec![inf; 1 << n];
    dp[0] = 0_i64;
    for _ in 0..k {
        let mut next = vec![inf; 1 << n];
        for s in 1_usize..1 << n {
            let mut t = s;
            while t != 0 {
                next[s] = next[s].min(dp[s - t].max(dist[t]));
                t = (t - 1) & s;
            }
        }
        dp = next;
    }

    let ans = dp[(1 << n) - 1];
    println!("{}", ans);
}
