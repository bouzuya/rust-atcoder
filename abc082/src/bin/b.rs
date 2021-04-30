use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    };
    s.sort();
    t.sort();
    t.reverse();
    let ans = s < t;
    println!("{}", if ans { "Yes" } else { "No" });
}
