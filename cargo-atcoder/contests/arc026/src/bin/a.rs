use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };
    let ans = if n <= 5 { b * n } else { a * (n - 5) + b * 5 };
    println!("{}", ans);
}
