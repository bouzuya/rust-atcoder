use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    };
    if s.len() == 3 {
        s.reverse();
    }
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
