use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    };
    let th = (h * 60_f64 + m) / (60_f64 * 12_f64) * 2_f64 * std::f64::consts::PI;
    let tm = m / 60_f64 * 2_f64 * std::f64::consts::PI;
    let dx = th.cos() * a - tm.cos() * b;
    let dy = th.sin() * a - tm.sin() * b;
    let ans = (dx * dx + dy * dy).sqrt();
    println!("{}", ans);
}
