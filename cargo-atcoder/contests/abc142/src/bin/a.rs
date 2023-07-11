use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let e = n / 2;
    let o = n - e;
    let ans = o as f64 / n as f64;
    println!("{}", ans);
}
