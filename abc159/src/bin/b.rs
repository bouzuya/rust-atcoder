use proconio::input;
use proconio::marker::Chars;

fn is_palindrome(s: &[char]) -> bool {
    (0..s.len() / 2).all(|i| s[i] == s[s.len() - i - 1])
}

fn main() {
    input! {
        s: Chars
    };
    let ans = is_palindrome(&s)
        && is_palindrome(&s[..(s.len() - 1) / 2])
        && is_palindrome(&s[(s.len() + 3) / 2 - 1..]);
    println!("{}", if ans { "Yes" } else { "No" });
}
