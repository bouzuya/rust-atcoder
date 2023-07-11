use proconio::input;

fn main() {
    input! {
        r: f64,
        d: f64,
    };
    let pi = std::f64::consts::PI;
    let ans = r * r * pi * 2_f64 * pi * d;
    println!("{}", ans);
}
