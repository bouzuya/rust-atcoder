use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut dp = vec![false; k + 1];
    dp[0] = false;
    for i in 1..=k {
        dp[i] = a
            .iter()
            .copied()
            .any(|a_j| if i < a_j { false } else { !dp[i - a_j] });
    }
    let ans = dp[k];
    println!("{}", if ans { "First" } else { "Second" });
}
