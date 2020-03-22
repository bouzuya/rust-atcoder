use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    for i in 0..(s.len() - 1) / 2 {
        if s[i] != s[s.len() - i - 1] {
            println!("No");
            return;
        }
    }
    let t = &s[0..(s.len() - 1) / 2];
    for i in 0..(t.len() - 1) / 2 + 1 {
        if t[i] != t[t.len() - i - 1] {
            println!("No");
            return;
        }
    }
    let u = &s[(s.len() + 3) / 2 - 1..s.len()];
    for i in 0..(u.len() - 1) / 2 + 1 {
        if u[i] != u[u.len() - i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
