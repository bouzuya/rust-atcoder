use proconio::input;

fn main() {
    input! {
        n: usize,
        mut wsv: [(usize, usize, usize); n],
    }

    wsv.sort_by(|&(w1, s1, v1), &(w2, s2, v2)| {
        let (w1, s1, _) = (w1 as i64, s1 as i64, v1 as i64);
        let (w2, s2, _) = (w2 as i64, s2 as i64, v2 as i64);
        s2.min(s1 - w2).cmp(&s1.min(s2 - w1))
    });

    let max_j = 10_000 * 2;
    let mut dp = vec![0_usize; max_j + 1];
    for (w, s, v) in wsv {
        let mut next = vec![0_usize; max_j + 1];
        for j in 0..=max_j {
            next[j] = next[j].max(dp[j]);
            if j <= s {
                next[j + w] = next[j + w].max(dp[j] + v);
            }
        }
        dp = next;
    }

    let ans = dp.iter().copied().max().unwrap();
    println!("{}", ans);
}
