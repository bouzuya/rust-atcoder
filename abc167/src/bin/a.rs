use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut ans = true;
    for i in 0..s.len() {
        if s[i] != t[i] {
            ans = false;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
