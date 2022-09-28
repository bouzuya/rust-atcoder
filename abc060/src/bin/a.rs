use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars,
    };
    let ans = a.last() == b.first() && b.last() == c.first();
    println!("{}", if ans { "YES" } else { "NO" });
}
