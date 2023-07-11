use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut ok = false;
    for i in 0..n - 2 {
        if s[i] == s[i + 1] && s[i] == s[i + 2] {
            ok = true;
        }
    }
    let ans = ok;
    println!("{}", if ans { "Yes" } else { "No" });
}
