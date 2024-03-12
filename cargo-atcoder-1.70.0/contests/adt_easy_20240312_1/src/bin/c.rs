use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().any(char::is_ascii_uppercase)
        && s.iter().any(char::is_ascii_lowercase)
        && s.iter().copied().collect::<HashSet<char>>().len() == s.len();
    println!("{}", if ans { "Yes" } else { "No" });
}
