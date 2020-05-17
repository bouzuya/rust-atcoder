use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: i64,
        m: i64,
    };
    let a1 = (-h % 12) as f64 * 360_f64 / 12_f64
        + ((-m % 60) as f64 * 360_f64 / 60_f64 / 12_f64)
        + 90_f64;
    let b1 = (-m % 60) as f64 * 360_f64 / 60_f64 + 90_f64;
    let a2 = a1 * (2_f64 * std::f64::consts::PI) / 360_f64;
    let b2 = b1 * (2_f64 * std::f64::consts::PI) / 360_f64;
    let x = a2.cos() * a - b2.cos() * b;
    let y = a2.sin() * a - b2.sin() * b;
    let ans = (x * x + y * y).sqrt();
    println!("{}", ans);
}
