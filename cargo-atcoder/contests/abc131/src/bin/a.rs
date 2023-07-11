use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s
        .iter()
        .enumerate()
        .skip(1)
        .any(|(i, &s_i)| s[i - 1] == s_i);
    println!("{}", if ans { "Bad" } else { "Good" });
}
