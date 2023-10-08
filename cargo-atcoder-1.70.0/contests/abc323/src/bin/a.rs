use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s
        .iter()
        .copied()
        .enumerate()
        .all(|(i, s_i)| i % 2 == 0 || s_i == '0');
    println!("{}", if ans { "Yes" } else { "No" });
}
