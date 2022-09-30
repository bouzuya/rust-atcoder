use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let f = |s: &[char]| -> bool {
        let t = s.iter().copied().rev().collect::<Vec<char>>();
        s == t.as_slice()
    };
    let ans = f(&s) && f(&s[0..n / 2]) && f(&s[n / 2 + 1..]);
    println!("{}", if ans { "Yes" } else { "No" });
}
