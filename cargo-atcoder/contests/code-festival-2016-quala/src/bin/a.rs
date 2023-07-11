use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let s1 = s[0..4].iter().collect::<String>();
    let s2 = s[4..].iter().collect::<String>();
    println!("{} {}", s1, s2);
}
