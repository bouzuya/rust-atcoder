use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().filter(|&&c| c == '1').count();
    println!("{}", ans);
}
