use std::cmp;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        _: Chars,
        a: [i64; n + 1],
    };
    let mut min_b = 1_000_000;
    for i in 0..n {
        min_b = cmp::min(min_b, (a[i] - a[i + 1]).abs());
    }
    let k = min_b;
    println!("{}", k);
    for i in 0..k {
        let mut s = String::new();
        for (j, a_j) in a.iter().copied().enumerate() {
            s.push_str(
                format!(
                    "{}{}",
                    (a_j + i) / k,
                    if j == a.len() - 1 { '\n' } else { ' ' }
                )
                .as_str(),
            );
        }
        print!("{}", s);
    }
}
