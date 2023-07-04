use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }

    let mut score = vec![0_i64; 1 << n];
    for (s, score_s) in score.iter_mut().enumerate() {
        for (i, a_i) in a.iter().enumerate() {
            for j in i + 1..n {
                if ((s & (1 << i)) != 0) && ((s & (1 << j)) != 0) {
                    *score_s += a_i[j];
                }
            }
        }
    }

    let mut dp = vec![0_i64; 1 << n];
    for s in 0_usize..(1 << n) {
        // 集合 s の部分集合 t を走査
        let mut prev = None;
        let mut t = s;
        while prev != Some(t) {
            t &= s;
            dp[s] = dp[s].max(dp[t] + score[s ^ t]);
            prev = Some(t);
            t = t.saturating_sub(1);
        }
    }

    let ans = dp[(1 << n) - 1];
    println!("{}", ans);
}
