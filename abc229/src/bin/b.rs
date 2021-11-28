use proconio::{input, marker::Bytes};

fn main() {
    input! {
        a: Bytes,
        b: Bytes,
    };

    for (&a_i, &b_i) in a.iter().rev().zip(b.iter().rev()) {
        if (a_i - b'0') + (b_i - b'0') > 9 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
