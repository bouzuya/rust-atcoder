use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut a: Chars,
        mut b: Chars,
    };
    a.reverse();
    b.reverse();
    for i in 0..a.len().min(b.len()) {
        if a[i] as u8 - b'0' + b[i] as u8 - b'0' >= 10 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
