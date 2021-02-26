use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = b - a - if a < 0 && b > 0 { 1 } else { 0 };
    println!("{}", ans);
}
