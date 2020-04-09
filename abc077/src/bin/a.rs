use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c1: Chars,
        c2: Chars
    };
    let ans = if c1[0] == c2[2] && c1[1] == c2[1] && c1[2] == c2[0] {
        "YES"
    } else {
        "NO"
    };
    println!("{}", ans);
}
