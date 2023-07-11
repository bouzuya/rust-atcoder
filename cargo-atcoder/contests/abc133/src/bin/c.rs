use proconio::input;
use std::cmp;

fn main() {
    input! {
        l: usize,
        r: usize,
    };
    let mut min_x = 2019 - 1;
    for i in l..=r {
        for j in i + 1..=r {
            let x = ((i % 2019) * (j % 2019)) % 2019;
            min_x = cmp::min(min_x, x);
            if min_x == 0 {
                println!("{}", x);
                return;
            }
        }
    }
    println!("{}", min_x);
}
