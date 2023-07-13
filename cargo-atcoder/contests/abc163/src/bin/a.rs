use proconio::input;

fn main() {
    input! {
        r: f64,
    };
    let ans = 2_f64 * std::f64::consts::PI * r;
    println!("{}", ans);
}
