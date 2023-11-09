use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.into_iter().enumerate().all(|(i, c)| {
        if i % 2 == 0 {
            c.is_ascii_lowercase()
        } else {
            c.is_ascii_uppercase()
        }
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
