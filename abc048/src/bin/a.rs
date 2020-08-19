use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: Chars,
        s: Chars,
        _: Chars,
    };
    let ans = format!("A{}C", s[0]);
    println!("{}", ans);
}
