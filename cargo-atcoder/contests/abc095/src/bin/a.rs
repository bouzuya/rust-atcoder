use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let ans = s.iter().filter(|&&c| c == 'o').count() * 100 + 700;
    println!("{}", ans);
}
