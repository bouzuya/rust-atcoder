use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut sum = 0_usize;
    for x in 1..=n {
        sum += x * 10_000;
    }
    let ans = sum as f64 / n as f64;
    println!("{}", ans);
}
