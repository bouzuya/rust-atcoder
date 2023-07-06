use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let modp = 1_000_000_007_usize;
    let mut dp = vec![1_usize; n];
    for (i, s_i) in s.iter().copied().enumerate() {
        let mut next = vec![0_usize; n];
        let c = std::iter::once(0)
            .chain(dp.iter().scan(0, |acc, &i| {
                *acc += i;
                Some(*acc)
            }))
            .collect::<Vec<usize>>();
        for j in 0..n - i {
            if s_i == '<' {
                next[j] += c[n - i] - c[j + 1];
                next[j] %= modp;
            } else {
                next[j] += c[j + 1] - c[0];
                next[j] %= modp;
            }
        }
        dp = next;
    }

    let ans = dp[0];
    println!("{}", ans);
}
