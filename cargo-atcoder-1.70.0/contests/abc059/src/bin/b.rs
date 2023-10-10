use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
    };
    let ans = match a.len().cmp(&b.len()) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.cmp(&b),
        Ordering::Greater => Ordering::Greater,
    };
    println!(
        "{}",
        match ans {
            Ordering::Less => "LESS",
            Ordering::Equal => "EQUAL",
            Ordering::Greater => "GREATER",
        }
    );
}
