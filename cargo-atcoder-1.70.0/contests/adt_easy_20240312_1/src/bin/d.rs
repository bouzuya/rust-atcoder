use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let b1 = s.iter().position(|c| c == &'B').unwrap();
    let b2 = b1 + 1 + s[b1 + 1..].iter().position(|c| c == &'B').unwrap();
    let r1 = s.iter().position(|c| c == &'R').unwrap();
    let r2 = r1 + 1 + s[r1 + 1..].iter().position(|c| c == &'R').unwrap();
    let k = s.iter().position(|c| c == &'K').unwrap();
    let ans = (b1 % 2 != b2 % 2) && (r1..r2).contains(&k);
    println!("{}", if ans { "Yes" } else { "No" });
}
