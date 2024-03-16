use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.len() >= 3
        && s.starts_with(&['<'])
        && s.ends_with(&['>'])
        && s.iter().skip(1).take(s.len() - 2).all(|&c| c == '=');
    println!("{}", if ans { "Yes" } else { "No" });
}
