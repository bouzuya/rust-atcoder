use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s[2] == s[3] && s[4] == s[5];
    println!("{}", if ans { "Yes" } else { "No" });
}
