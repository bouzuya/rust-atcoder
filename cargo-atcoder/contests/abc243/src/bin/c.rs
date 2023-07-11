use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        s: Chars,
    };
    let mut ys = HashMap::new();
    for (i, (x, y)) in xy.iter().copied().enumerate() {
        let entry = ys.entry(y).or_insert((1_000_000_001, -1));
        match s[i] {
            'L' => entry.1 = entry.1.max(x),
            'R' => entry.0 = entry.0.min(x),
            _ => unreachable!(),
        }
    }

    for (_, (l, r)) in ys {
        if l < r {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
