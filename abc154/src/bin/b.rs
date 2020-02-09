use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let ans: String = s.iter().map(|_| 'x').collect();
    println!("{}", ans);
}
