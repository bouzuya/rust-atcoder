use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    };

    let mut ng = 90_f64;
    let mut ok = 0_f64;
    for _ in 0..100 {
        let m = (ok + ng) / 2_f64;
        let t = (90_f64 - m) * std::f64::consts::PI / 180_f64;
        let o = if b * t.tan() <= a {
            a * b * b * t.tan() / 2_f64 >= x
        } else {
            let t = m * std::f64::consts::PI / 180_f64;
            a * a * t.tan() * a / 2_f64 + (b - a * t.tan()) * a * a >= x
        };
        if o {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}
