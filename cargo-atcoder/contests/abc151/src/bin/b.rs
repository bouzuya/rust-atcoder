use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n - 1],
    };
    let inf = k + 1;
    let mut min_x = inf;
    let sum = a.iter().sum::<usize>();
    for x in 0..=k {
        if sum + x >= m * n {
            min_x = min(min_x, x);
        }
    }
    println!("{}", if min_x == inf { -1 } else { min_x as i64 });
}
