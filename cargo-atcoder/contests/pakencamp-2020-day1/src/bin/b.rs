use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    };
    let ans = x.iter().max().unwrap();
    println!("{}", ans);
}
