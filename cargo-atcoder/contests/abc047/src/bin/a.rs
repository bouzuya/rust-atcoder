use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let ans = a + b == c || a + c == b || a == b + c;
    println!("{}", if ans { "Yes" } else { "No" });
}
