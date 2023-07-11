use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let ans = match (s[0] == s[n - 1], n % 2 == 0) {
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => true,
    };
    println!("{}", if ans { "First" } else { "Second" });
}
