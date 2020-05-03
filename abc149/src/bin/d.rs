use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        k: usize,
        r: u64,
        s: u64,
        p: u64,
        t: Chars,
    };
    let mut dp = vec![(0_u64, 0_u64, 0_u64); k];
    for (i, &c) in t.iter().enumerate() {
        let (pr, ps, pp) = dp[i % k];
        dp[i % k] = (
            std::cmp::max(ps, pp) + if c == 's' { r } else { 0 },
            std::cmp::max(pr, pp) + if c == 'p' { s } else { 0 },
            std::cmp::max(pr, ps) + if c == 'r' { p } else { 0 },
        );
    }
    let ans = dp
        .iter()
        .map(|i| std::cmp::max(i.0, std::cmp::max(i.1, i.2)))
        .sum::<u64>();
    println!("{}", ans);
}
