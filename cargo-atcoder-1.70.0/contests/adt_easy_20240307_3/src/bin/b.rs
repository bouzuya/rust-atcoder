use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    for i in (2..=16).step_by(2) {
        let i = i - 1;
        if s[i] != '0' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
