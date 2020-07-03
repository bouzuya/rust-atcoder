use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars,
    };
    let ans = a[a.len() - 1] == b[0] && b[b.len() - 1] == c[0];
    println!("{}", if ans { "YES" } else { "NO" });
}
