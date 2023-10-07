use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    for i in 1..16 {
        if i % 2 == 0 {
            continue;
        }
        if s[i] != '0' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
