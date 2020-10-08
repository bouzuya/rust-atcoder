use proconio::input;

fn main() {
    use std::cmp::max;
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = max(a * c, max(a * d, max(b * c, b * d)));
    println!("{}", ans);
}
