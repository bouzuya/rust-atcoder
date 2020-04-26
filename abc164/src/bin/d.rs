use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! { s: Bytes };
    let ds: Vec<u8> = s.iter().map(|b| b - b'0').collect();
    let n = ds.len();
    let mut count = 0;
    let mut b = 1_usize;
    let mut r = 0_usize;
    let mut dp = vec![0_usize; 2019];
    dp[0] = 1;
    for i in (0..n).rev() {
        let d = ds[i] as usize;
        r = (d * b + r) % 2019;
        b = (b * 10) % 2019;
        count += dp[r];
        dp[r] += 1;
    }
    let ans = count;
    println!("{}", ans);
}
