use proconio::input;

fn main() {
    input! {
        n: f64,
        (x0, y0): (f64, f64),
        (x2, y2): (f64, f64),
    };
    let cx = (x2 - x0) / 2_f64 + x0;
    let cy = (y2 - y0) / 2_f64 + y0;
    let dx = x0 - cx;
    let dy = y0 - cy;
    let t = (360_f64 / n) * 2_f64 * std::f64::consts::PI / 360_f64;
    let x1 = dx * t.cos() - dy * t.sin() + cx;
    let y1 = dy * t.cos() + dx * t.sin() + cy;
    println!("{} {}", x1, y1);
}
