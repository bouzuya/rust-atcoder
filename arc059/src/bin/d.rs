use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    for i in 0..s.len() - 1 {
        if s[i] == s[i + 1] {
            println!("{} {}", i + 1, i + 2);
            return;
        }
    }
    for i in 0..s.len() - 2 {
        if s[i] == s[i + 2] {
            println!("{} {}", i + 1, i + 3);
            return;
        }
    }
    println!("-1 -1");
}
