use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    if s[0] != 'A' {
        println!("WA");
        return;
    }
    s[0] = 'a';
    if s[2..s.len() - 1].iter().filter(|&&c| c == 'C').count() != 1 {
        println!("WA");
        return;
    }
    let i = s[2..s.len() - 1].iter().position(|&c| c == 'C').unwrap() + 2;
    s[i] = 'c';
    if !s.iter().all(|c| c.is_ascii_lowercase()) {
        println!("WA");
        return;
    }
    println!("AC");
}
