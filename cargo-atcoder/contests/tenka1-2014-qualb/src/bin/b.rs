use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: [Chars; n],
    };
    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1;
    for i in 0..s.len() {
        for t_j in t.iter() {
            if i + t_j.len() <= s.len() && s[i..i + t_j.len()] == t_j[0..t_j.len()] {
                dp[i + t_j.len()] += dp[i];
                dp[i + t_j.len()] %= 1_000_000_007;
            }
        }
    }
    let ans = dp[s.len()];
    println!("{}", ans);
}
