use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = if let Some(i) = s.iter().position(|c| c == &'C') {
        s[i + 1..].iter().any(|c| c == &'F')
    } else {
        false
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
