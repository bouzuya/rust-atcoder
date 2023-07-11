use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
    };
    let mut dp = vec![0];
    for i in 0..n {
        let p = dp.clone();
        input! {
            a_i: [u64; i + 1],
        }
        for (j, &a_ij) in a_i.iter().enumerate() {
            dp[j] = max(if j > 0 { p[j - 1] + a_ij } else { 0 }, p[j] + a_ij);
        }
        dp.push(a_i[a_i.len() - 1]);
    }
    let ans = *dp.iter().max().unwrap();
    println!("{}", ans);
}
