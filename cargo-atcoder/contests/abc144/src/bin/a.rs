use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = if a > 9 || b > 9 { -1 } else { a * b };
    println!("{}", ans);
}
