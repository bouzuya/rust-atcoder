use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: usize,
        s: Chars
    };
    if s.len() <= k {
        println!("{}", s.iter().collect::<String>());
    } else {
        println!("{}{}", s[0..k].iter().collect::<String>(), "...");
    }
}
