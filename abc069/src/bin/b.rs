use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    print!("{}{}{}", s[0], s.len() - 2, s[s.len() - 1]);
}
