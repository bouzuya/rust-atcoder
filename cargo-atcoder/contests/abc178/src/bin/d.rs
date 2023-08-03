use proconio::input;

fn cumsum(a: &[usize]) -> Vec<usize> {
    std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            *acc %= 1_000_000_007;
            Some(*acc)
        }))
        .collect()
}

fn main() {
    input! {
        s: usize,
    };
    let mut ans = 0_usize;
    let mut dp = vec![0_usize; s + 1];
    dp[0] = 1;
    let mut cs = cumsum(&dp);
    for _ in 0..s {
        let mut next = vec![0_usize; s + 1];
        for j in 3..=s {
            next[j] += cs[j - 3 + 1] - cs[0];
        }
        dp = next;
        cs = cumsum(&dp);
        ans += dp[s];
        ans %= 1_000_000_007;
    }

    println!("{}", ans);
}
