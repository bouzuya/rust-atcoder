use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    };
    let mut dp = vec![vec![vec![None; d]; k + 1]; n + 1];
    dp[0][0][0] = Some(0_usize);
    for (i, a_i) in a.iter().copied().enumerate() {
        for j in 0..=k {
            for r in 0..d {
                if let Some(v) = dp[i][j][r] {
                    if j < k {
                        // 選ぶ場合
                        let nv = v + a_i;
                        let nr = nv % d;
                        dp[i + 1][j + 1][nr] = Some(match dp[i + 1][j + 1][nr] {
                            Some(ov) => ov.max(nv),
                            None => nv,
                        });
                    }

                    // 選ばない場合
                    dp[i + 1][j][r] = Some(match dp[i + 1][j][r] {
                        Some(ov) => ov.max(v),
                        None => v,
                    });
                }
            }
        }
    }

    let ans = dp[n][k][0].map(|x| x as i64).unwrap_or(-1);
    println!("{:?}", ans);
}
