use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    for i in 0..15 {
        if i >= s.len() {
            s.push('o');
        }
    }
    let ans = s.iter().filter(|&&c| c == 'o').count() >= 8;
    println!("{}", if ans { "YES" } else { "NO" });
}
