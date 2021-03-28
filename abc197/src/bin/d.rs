use num::Complex;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x0: i64,
        y0: i64,
        x2: i64,
        y2: i64,
    };
    // let a = (x2 - x0) as f64 / 2_f64 + x0 as f64;
    // let b = (y2 - y0) as f64 / 2_f64 + y0 as f64;
    // let x = x0 as f64 - a;
    // let y = y0 as f64 - b;
    // let t = (360_f64 / n as f64) * 2_f64 * std::f64::consts::PI / 360_f64;
    // let x1 = x as f64 * t.cos() - y as f64 * t.sin() + a;
    // let y1 = y as f64 * t.cos() + x as f64 * t.sin() + b;
    // println!("{} {}", x1, y1);

    // 複素数 <https://youtu.be/mk8mGugZREg?t=1815>
    let s = Complex::new(x0 as f64, y0 as f64);
    let t = Complex::new(x2 as f64, y2 as f64);
    let o = (s + t) / Complex::new(2 as f64, 0 as f64);
    let t = 2_f64 * std::f64::consts::PI / n as f64;
    let r = Complex::new(t.cos(), t.sin());
    let ans = o + (s - o) * r;
    println!("{} {}", ans.re, ans.im);
}
