use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let ans = ((n - k) * (k - 1) * 6 + (n - 1) * 3 + 1) as f64 / (n * n * n) as f64;
    println!("{}", ans);
}
