use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let modp = 1_000_000_007;
    let mut dp = (1, 1);
    for _ in 0..n - 2 {
        dp = (dp.1, (dp.0 + dp.1) % modp);
    }
    let ans = dp.1;
    println!("{}", ans);
}
