use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = (s.contains(&'N') == s.contains(&'S')) && (s.contains(&'E') == s.contains(&'W'));
    println!("{}", if ans { "Yes" } else { "No" });
}
