use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let ans = s[s.len() - 1] == 'T';
    println!("{}", if ans { "YES" } else { "NO" });
}
