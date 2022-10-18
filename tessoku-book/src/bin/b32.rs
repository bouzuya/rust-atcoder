use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    };
    let mut dp = vec![false; n + 1];
    dp[0] = false;
    for i in 1..=n {
        dp[i] = a
            .iter()
            .copied()
            .any(|a_j| if i >= a_j { !dp[i - a_j] } else { false });
    }
    let ans = dp[n];
    println!("{}", if ans { "First" } else { "Second" });
}
