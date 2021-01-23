use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: Chars,
    };
    let ans = c[0] == c[1] && c[1] == c[2];
    println!("{}", if ans { "Won" } else { "Lost" });
}
