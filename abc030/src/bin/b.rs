use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    };
    let a = ((n % 12) * 360) as f64 / 12_f64 + (m * 360) as f64 / 60_f64 / 12_f64;
    let b = (m * 360) as f64 / 60_f64;
    let x = (a - b).abs();
    let ans = x.min(360_f64 - x);
    println!("{}", ans);
}
