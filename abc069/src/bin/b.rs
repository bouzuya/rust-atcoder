use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    println!("{}{}{}", s.first().unwrap(), s.len() - 2, s.last().unwrap());
}
