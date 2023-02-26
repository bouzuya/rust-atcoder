use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().position(|c| c.is_ascii_uppercase()).unwrap() + 1;
    println!("{}", ans);
}
