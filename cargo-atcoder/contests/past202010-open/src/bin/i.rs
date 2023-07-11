use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let b = std::iter::once(&0)
        .chain(a.iter())
        .chain(a.iter())
        .scan(0, |acc, i| {
            *acc += i;
            Some(*acc)
        })
        .collect::<Vec<i64>>();
    let sum = a.iter().sum::<i64>();

    let mut min_value = sum;
    for i in 0..n {
        let mut l = i;
        let mut r = 2 * n;
        while r - l > 1 {
            let m = l + (r - l) / 2;
            let x = b[m] - b[i];
            let y = sum - x;
            if x - y > 0 {
                r = m;
            } else {
                l = m;
            }
            min_value = min(min_value, (x - y).abs());
        }
    }
    let ans = min_value;
    println!("{}", ans);
}
