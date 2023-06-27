use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by(|&(a1, b1), &(a2, b2)| {
        (a1 as f64 / b1 as f64)
            .partial_cmp(&(a2 as f64 / b2 as f64))
            .unwrap()
    });
    ab.reverse();

    let mut dp = vec![0_usize; h + 1];
    for (a_i, b_i) in ab {
        let mut next = vec![0_usize; h + 1];
        for j in 0..=h {
            next[j.saturating_sub(b_i)] = next[j.saturating_sub(b_i)].max(dp[j] + a_i * j);
            next[j] = next[j].max(dp[j]);
        }
        dp = next;
    }

    let ans = dp.iter().copied().max().unwrap();
    println!("{}", ans);
}
