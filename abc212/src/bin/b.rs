use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };
    if x.iter().copied().collect::<BTreeSet<char>>().len() == 1 {
        println!("Weak");
        return;
    }
    let b = (x[0] as u8 - b'0') as usize;
    for i in 0..4 {
        if (b + i) % 10 != ((x[i] as u8 - b'0') as usize) {
            println!("Strong");
            return;
        }
    }
    println!("Weak");
}
