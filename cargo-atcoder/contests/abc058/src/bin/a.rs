use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let ans = b - a == c - b;
    println!("{}", if ans { "YES" } else { "NO" });
}
