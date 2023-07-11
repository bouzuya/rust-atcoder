use proconio::input;

fn main() {
    input! {
        n: f64,
    };
    let ans = (9_f64 / 5_f64 * n) + 32_f64;
    println!("{}", ans);
}
