use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    };
    let ans = n[0] == n[2];
    println!("{}", if ans { "Yes" } else { "No" });
}
