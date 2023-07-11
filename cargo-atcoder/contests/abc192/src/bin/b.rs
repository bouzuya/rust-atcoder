use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    for (i, c) in s.iter().enumerate() {
        let b = if i % 2 != 0 {
            c.is_uppercase()
        } else {
            c.is_lowercase()
        };
        if !b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
