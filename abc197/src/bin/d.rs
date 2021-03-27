use proconio::input;

fn main() {
    input! {
        n: usize,
        x0: i64,
        y0: i64,
        x2: i64,
        y2: i64,
    };
    let a = (x2 - x0) as f64 / 2_f64 + x0 as f64;
    let b = (y2 - y0) as f64 / 2_f64 + y0 as f64;
    let x = x0 as f64 - a;
    let y = y0 as f64 - b;
    let t = (360_f64 / n as f64) * 2_f64 * std::f64::consts::PI / 360_f64;
    let x1 = x as f64 * t.cos() - y as f64 * t.sin() + a;
    let y1 = y as f64 * t.cos() + x as f64 * t.sin() + b;
    println!("{} {}", x1, y1);
}
