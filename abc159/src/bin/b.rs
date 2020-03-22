use proconio::input;
use proconio::marker::Chars;

fn is_palindrome(s: &[char]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    return true;
}

fn main() {
    input! {
        s: Chars
    };
    let ans = is_palindrome(&s)
        && is_palindrome(&s[0..(s.len() - 1) / 2])
        && is_palindrome(&s[(s.len() + 3) / 2 - 1..s.len()]);
    println!("{}", if ans { "Yes" } else { "No" });
}
