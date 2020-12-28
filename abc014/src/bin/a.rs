use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let r = a % b;
    let ans = if r == 0 { 0 } else { b - r };
    println!("{}", ans);
}
