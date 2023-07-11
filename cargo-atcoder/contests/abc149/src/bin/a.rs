use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let ans = t.iter().chain(s.iter()).collect::<String>();
    println!("{}", ans);
}
