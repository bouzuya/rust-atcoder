use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        s: Chars,
    };

    let mut min_x = 753;
    for i in 0..=s.len() - 3 {
        let x = s[i..i + 3]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        min_x = cmp::min(min_x, (x - 753).abs());
    }

    let ans = min_x;
    println!("{}", ans);
}
