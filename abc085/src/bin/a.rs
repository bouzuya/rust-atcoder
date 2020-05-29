use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    println!("2018{}", s[4..].iter().collect::<String>());
}
