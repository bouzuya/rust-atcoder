use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = "x".repeat(s.len());
    println!("{}", ans);
}
