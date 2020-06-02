use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64
    };
    let ans = n % 500 <= a;
    println!("{}", if ans { "Yes" } else { "No" });
}
