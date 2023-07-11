use std::{cmp::min, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut map = BTreeMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1_usize;
    }
    let mut sum = 0_usize;
    let mut count = k;
    for x in 0..=n {
        match map.get(&x) {
            None => {
                sum += count * x;
                break;
            }
            Some(&count_x) => {
                let c_x = min(count, count_x);
                let empty = count - c_x;
                sum += empty * x;
                count -= empty;
            }
        }
    }

    let ans = sum;
    println!("{}", ans);
}
