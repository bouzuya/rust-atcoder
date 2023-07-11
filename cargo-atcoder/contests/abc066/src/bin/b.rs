use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    for l in (0..s.len()).rev() {
        if l % 2 != 0 {
            continue;
        }
        if s[0..l / 2] == s[l / 2..l] {
            println!("{}", l);
            return;
        }
    }
}
