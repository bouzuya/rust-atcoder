use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    if s[0] != s[n - 1] {
        println!("1");
        return;
    }
    let c = s[0];
    for i in 1..n - 1 {
        if c != s[i] && s[i + 1] != c {
            println!("2");
            return;
        }
    }
    println!("-1");
}
