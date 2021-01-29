use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: char,
        s: Chars,
    };
    let ans = s.into_iter().filter(|&c| c != x).collect::<String>();
    println!("{}", ans);
}
