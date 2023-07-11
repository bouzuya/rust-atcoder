use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let ans = if s[..] == t[..s.len()] { "Yes" } else { "No" };
    println!("{}", ans);
}
