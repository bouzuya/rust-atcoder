use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s[s.len() / 2];
    println!("{}", ans);
}
