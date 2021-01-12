use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let ans = s[0..n - "FESTIVAL".len()].iter().collect::<String>();
    println!("{}", ans);
}
