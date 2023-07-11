use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let ans = s.contains(&'o') && !s.contains(&'x');
    println!("{}", if ans { "Yes" } else { "No" });
}
