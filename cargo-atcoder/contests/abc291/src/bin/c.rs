use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut set = HashSet::new();
    let mut pos = (0_i64, 0_i64);
    set.insert(pos);
    for c in s {
        let (x, y) = pos;
        let next = match c {
            'R' => (x + 1, y),
            'L' => (x - 1, y),
            'U' => (x, y + 1),
            'D' => (x, y - 1),
            _ => unreachable!(),
        };
        if !set.insert(next) {
            println!("Yes");
            return;
        }
        pos = next;
    }
    println!("No");
}
