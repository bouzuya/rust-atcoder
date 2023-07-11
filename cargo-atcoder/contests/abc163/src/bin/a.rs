use proconio::input;

fn main() {
    input! {
        r: f64,
    };
    let ans = 2.0 * std::f64::consts::PI * r;
    println!("{}", ans);
}
