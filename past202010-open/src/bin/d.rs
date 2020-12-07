use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut counts = vec![0];
    for i in 0..n {
        match s[i] {
            '.' => {
                let l = counts.len();
                counts[l - 1] += 1;
            }
            '#' => {
                counts.push(0);
            }
            _ => unreachable!(),
        }
    }
    let l = counts.len();
    let count_l = counts[0];
    let count_m = *counts[1..l - 1].iter().max().unwrap_or(&0);
    let count_r = counts[l - 1];
    let l = count_l;
    let r = count_r + max(0, count_m - (count_l + count_r));
    println!("{} {}", l, r);
}
