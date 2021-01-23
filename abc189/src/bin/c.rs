use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
         a: [usize; n],
    };
    let mut max_x = 0;
    for &x in a.iter() {
        let mut max_count = 0;
        let mut count = 0;
        for &a_i in a.iter() {
            if a_i >= x {
                count += 1;
                max_count = max(max_count, count);
            } else {
                count = 0;
            }
        }
        max_x = max(max_x, max_count * x);
    }
    println!("{}", max_x);
}
