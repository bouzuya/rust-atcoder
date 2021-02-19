use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        d: [i64; 7],
        j: [i64; 7],
    };
    let ans = d
        .iter()
        .zip(j.iter())
        .map(|(&d_i, &j_i)| max(d_i, j_i))
        .sum::<i64>();
    println!("{}", ans);
}
