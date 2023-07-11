use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for (i, s_i) in s.iter().enumerate() {
        match s_i.as_str() {
            "AND" => {
                dp[i + 1] += dp[i]; // t -> t
            }
            "OR" => {
                dp[i + 1] += dp[i]; // t -> t
                dp[i + 1] += dp[i]; // t -> f
                dp[i + 1] += 2_usize.pow((i + 1) as u32) - dp[i]; // f -> t
            }
            _ => unreachable!(),
        }
    }
    let ans = dp[n];
    println!("{}", ans);
}
