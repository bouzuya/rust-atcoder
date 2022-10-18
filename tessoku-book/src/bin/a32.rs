use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let mut dp = vec![true; n + 1];
    dp[0] = false;
    for i in 1..=n {
        dp[i] = match (i >= a, i >= b) {
            (false, false) => false,
            (false, true) => unreachable!(),
            (true, false) => !dp[i - a],
            (true, true) => !dp[i - a] || !dp[i - b],
        };
    }
    let ans = dp[n];
    println!("{}", if ans { "First" } else { "Second" });
}
