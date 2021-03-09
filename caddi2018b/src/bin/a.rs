use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    };
    let ans = n.iter().filter(|&&c| c == '2').count();
    println!("{}", ans);
}
