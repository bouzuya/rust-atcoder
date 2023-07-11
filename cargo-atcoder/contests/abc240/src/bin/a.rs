use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = a == b - 1 || (a == 1 && b == 10);
    println!("{}", if ans { "Yes" } else { "No" });
}
