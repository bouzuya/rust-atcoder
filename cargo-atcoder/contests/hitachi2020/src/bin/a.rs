use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    if n % 2 != 0 {
        println!("No");
        return;
    }
    for i in (0..n).step_by(2) {
        if s[i] != 'h' || s[i + 1] != 'i' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
