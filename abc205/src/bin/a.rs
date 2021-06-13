use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };
    let ans = b / 100_f64 * a;
    println!("{}", ans);
}
