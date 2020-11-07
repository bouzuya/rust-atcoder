use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    for (i, &c) in s.iter().enumerate() {
        print!(
            "{}",
            if i == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        );
    }
    println!();
}
