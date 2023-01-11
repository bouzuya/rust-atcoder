use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: usize,
        _b: usize,
        s: Chars,
    };
    let ans = s
        .into_iter()
        .enumerate()
        .all(|(i, c)| (i == a && c == '-') || (i != a && c.is_ascii_digit()));
    println!("{}", if ans { "Yes" } else { "No" });
}
