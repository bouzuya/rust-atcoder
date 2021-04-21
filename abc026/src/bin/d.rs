use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    };
    let mut l = 0_f64;
    let mut r = 1_000_000_000_f64;
    for _ in 0..100 {
        let t = l + (r - l) / 2_f64;
        let ft = a * t + b * (c * t * std::f64::consts::PI).sin();
        if ft > 100_f64 {
            r = t;
        } else {
            l = t;
        }
    }
    let ans = r;
    println!("{}", ans);
}
