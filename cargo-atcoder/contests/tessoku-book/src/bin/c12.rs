use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
    };

    let mut dp = vec![vec![vec![None; n + 2]; k + 1]; n + 1];
    dp[0][0][1] = Some(0);
    for r in 1..=n {
        for j in 0..=k {
            for l in 1..=r {
                let nj = j + 1;
                let nl = r + 1;
                match dp[r - 1][j][l] {
                    None => continue,
                    Some(v) => {
                        if nj <= k {
                            let chapter = l..=r;
                            let mut count = 0_usize;
                            for (a, b) in ab.iter().copied() {
                                if chapter.contains(&a) && chapter.contains(&b) {
                                    count += 1;
                                }
                            }
                            dp[r][nj][nl] = Some(match dp[r][nj][nl] {
                                Some(x) => x.max(v + count),
                                None => v + count,
                            });
                        }
                        dp[r][j][l] = Some(match dp[r][j][l] {
                            Some(x) => x.max(v),
                            None => v,
                        });
                    }
                }
            }
        }
    }

    let ans = dp[n][k].iter().max().unwrap().unwrap();
    println!("{}", ans);
}
