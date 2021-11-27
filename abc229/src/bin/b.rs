use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
    };

    for (&a_i, &b_i) in a.iter().rev().zip(b.iter().rev()) {
        if (a_i as u8 - b'0') + (b_i as u8 - b'0') > 9 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
