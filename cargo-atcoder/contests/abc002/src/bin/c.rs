use proconio::input;

fn f(a: f64, b: f64, c: f64, d: f64) -> f64 {
    (a * d - b * c).abs() / 2_f64
}

fn main() {
    input! {
        a: (f64, f64),
        b: (f64, f64),
        c: (f64, f64),
    };
    let ans = f(b.0 - a.0, b.1 - a.1, c.0 - a.0, c.1 - a.1);
    println!("{}", ans);
}
