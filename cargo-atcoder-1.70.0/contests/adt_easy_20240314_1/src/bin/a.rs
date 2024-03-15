use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().position(char::is_ascii_uppercase).unwrap() + 1;
    println!("{}", ans);
}
