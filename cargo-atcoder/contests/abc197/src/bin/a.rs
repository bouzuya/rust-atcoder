use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    };
    s.rotate_left(1);
    println!("{}", s.iter().collect::<String>());
}
