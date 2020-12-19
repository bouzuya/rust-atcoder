use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    };
    let mut min_a = 1_000;
    for i in 0..h {
        for j in 0..w {
            min_a = min(min_a, a[i][j]);
        }
    }

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            count += a[i][j] - min_a;
        }
    }

    let ans = count;
    println!("{}", ans);
}
