use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let ans = if n % 2 == 0 && (0..n / 2).all(|i| s[i] == s[n / 2 + i]) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
