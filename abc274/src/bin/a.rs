use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };
    let ans = ((b / a) * 1_000_f64).round() / 1_000_f64;
    println!("{:.3}", ans);
}
