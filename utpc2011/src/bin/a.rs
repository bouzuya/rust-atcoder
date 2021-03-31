use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        m: usize,
        n: usize,
        a: [[u8; n]; m],
    };
    let mut max_count = 0;
    for i in 0..m {
        let mut count = 0;
        for j in 0..n {
            if a[i][j] == 1 {
                count += 1;
            }
        }
        max_count = max(max_count, count);
    }
    let ans = max_count;
    println!("{}", ans);
}
