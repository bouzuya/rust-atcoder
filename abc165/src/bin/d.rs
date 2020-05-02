use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        n: i64,
    };
    let x = std::cmp::min(b - 1, n);
    let ans = a * x / b - a * (x / b);
    println!("{}", ans);
}
