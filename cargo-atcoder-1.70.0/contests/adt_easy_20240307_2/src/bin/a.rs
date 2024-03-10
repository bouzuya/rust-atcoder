use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans =
        s.first().unwrap().is_ascii_uppercase() && s.iter().skip(1).all(char::is_ascii_lowercase);
    println!("{}", if ans { "Yes" } else { "No" });
}
