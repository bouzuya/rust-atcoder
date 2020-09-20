use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let k1 = (n - k) * (k - 1) * 6;
    let k2 = (n - 1) * 3;
    let k3 = 1;
    let ans = (k1 + k2 + k3) as f64 / (n * n * n) as f64;
    println!("{}", ans);
}
