use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let ans = (0..n / 2).filter(|&i| s[i] != s[n - 1 - i]).count();
    println!("{}", ans);
}
