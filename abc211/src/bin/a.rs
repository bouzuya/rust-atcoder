use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };
    let c = (a - b) / 3_f64 + b;
    let ans = c;
    println!("{}", ans);
}
