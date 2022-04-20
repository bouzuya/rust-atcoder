use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = (s[0] == s[s.len() - 1]) == (s.len() % 2 == 0);
    println!("{}", if ans { "First" } else { "Second" });
}
