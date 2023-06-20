use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let modp = 1_000_000_007_usize;
    let mut dp = vec![0_usize; k + 1];
    dp[0] = 1_usize;
    for a_i in a.iter().copied() {
        let mut next = vec![0_usize; k + 1];
        let sum = std::iter::once(0)
            .chain(dp.iter().scan(0, |acc, &i| {
                *acc += i;
                *acc %= modp;
                Some(*acc)
            }))
            .collect::<Vec<usize>>();

        for j in 0..=k {
            next[j] = modp + sum[j + 1] - sum[j.saturating_sub(a_i)];
            next[j] %= modp;
        }
        dp = next;
    }
    let ans = dp[k];
    println!("{}", ans);
}
