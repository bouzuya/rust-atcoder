use std::cmp::max;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };
    let mut bci = vec![];
    for _ in 0..m {
        input! {
            b: i64,
            c: usize,
            i: [Usize1; c],
        }
        bci.push((b, c, i));
    }
    let mut max_score = 0;
    for bits in 0..1 << n {
        let is = (0..n)
            .filter(|i| (bits >> i) & 1 == 1)
            .collect::<Vec<usize>>();
        if is.len() > 9 {
            continue;
        }
        let mut score = a
            .iter()
            .enumerate()
            .filter(|&(i, _)| is.contains(&i))
            .map(|(_, &a_i)| a_i)
            .sum::<i64>();
        for (b, _, i) in bci.iter() {
            if i.iter().filter(|j| is.contains(j)).count() >= 3 {
                score += b.clone();
            }
        }
        max_score = max(max_score, score);
    }
    let ans = max_score;
    println!("{}", ans);
}
