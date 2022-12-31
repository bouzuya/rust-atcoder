use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
        k: usize,
    };
    let ds = n
        .iter()
        .copied()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();
    let mut dp = vec![vec![0_usize; k + 1]; 2];
    let d = ds[0];
    for x in (0..=9).rev() {
        if x > d {
            continue;
        }
        let ni = if x < d { 1 } else { 0 };
        let nj = if x != 0 { 1 } else { 0 };
        dp[ni][nj] += 1;
    }

    for d in ds.into_iter().skip(1) {
        let mut next = vec![vec![0_usize; k + 1]; 2];
        for j in 0..=k {
            // 0 -> 0 or 1
            for x in (0..=9).rev() {
                if x > d {
                    continue;
                }
                let ni = if x < d { 1 } else { 0 };
                let nj = if x != 0 { 1 } else { 0 } + j;
                if nj <= k {
                    next[ni][nj] += dp[0][j];
                }
            }

            // 1 -> 1
            for x in (0..=9).rev() {
                let ni = 1;
                let nj = if x != 0 { 1 } else { 0 } + j;
                if nj <= k {
                    next[ni][nj] += dp[1][j];
                }
            }
        }

        dp = next;
    }

    let ans = dp[0][k] + dp[1][k];
    println!("{}", ans);
}
