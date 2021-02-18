use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n: usize,
        color: [u8; n],
    };

    let mut count_b = 0;
    let mut count_r = 0;
    for &c_i in color.iter() {
        match c_i {
            0 => count_b += 1,
            1 => count_r += 1,
            _ => unreachable!("input"),
        }
    }
    if count_b == 0 || count_r == 0 {
        println!("{}", -1);
        return;
    }

    let mut max_count = 1;
    let mut count = 1;
    let mut prev = color[0];
    for &c_i in color.iter().chain(color.iter()).skip(1) {
        if prev == c_i {
            count += 1;
            max_count = max(max_count, count);
        } else {
            count = 1;
            prev = c_i;
        }
    }
    let ans = (max_count - 1) / 2 + 1;
    println!("{}", ans);
}
