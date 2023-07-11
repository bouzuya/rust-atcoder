use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };
    let ans = 100_f64 - b / a * 100_f64;
    println!("{}", ans);
}
