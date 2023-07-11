use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        h: i64,
        m: i64,
    };
    let da = 90_f64 + (h * 60 + m) as f64 / 2_f64;
    let db = 90_f64 + (m * 6) as f64;
    let ra = da * std::f64::consts::PI / 180_f64;
    let rb = db * std::f64::consts::PI / 180_f64;
    let xa = ra.cos() * a as f64;
    let ya = ra.sin() * a as f64;
    let xb = rb.cos() * b as f64;
    let yb = rb.sin() * b as f64;
    let ans = ((xa - xb).powf(2_f64) + (ya - yb).powf(2_f64)).sqrt();
    println!("{}", ans);
}
