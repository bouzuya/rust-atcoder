use proconio::input;

fn main() {
    use std::cmp::max;
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = max(max(max(a * c, a * d), b * c), b * d);
    println!("{}", ans);
}
