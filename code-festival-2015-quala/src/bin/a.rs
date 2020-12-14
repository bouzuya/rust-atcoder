use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    };
    let n = s.len();
    s[n - 1] = '5';
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
