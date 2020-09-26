use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let ans = s
        .iter()
        .zip(s.iter().rev())
        .take(s.len() / 2)
        .all(|(&s_i, &s_j)| s_i == '*' || s_j == '*' || s_i == s_j);
    println!("{}", if ans { "YES" } else { "NO" });
}
