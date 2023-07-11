use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let mut is_palindrome = true;
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            is_palindrome = false;
            break;
        }
    }
    let ans = is_palindrome;
    println!("{}", if ans { "YES" } else { "NO" });
}
