use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        d: i64,
    };
    let ans = (if d == 0 {
        1_f64 / n as f64
    } else {
        (2 * (n - d)) as f64 / (n * n) as f64
    }) * (m - 1) as f64;
    println!("{}", ans);
}
