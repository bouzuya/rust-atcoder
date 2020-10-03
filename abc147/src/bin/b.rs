use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let ans = s
        .iter()
        .zip(s.iter().rev())
        .take(s.len() / 2)
        .map(|(s_i, s_j)| if s_i == s_j { 0 } else { 1 })
        .sum::<i64>();
    println!("{}", ans);
}
