use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().enumerate().all(|(i, &s_i)| {
        if i == 4 - 1 {
            s_i == '-'
        } else {
            s_i.is_ascii_digit()
        }
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
