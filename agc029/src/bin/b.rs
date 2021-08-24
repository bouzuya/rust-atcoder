use std::{cmp::Reverse, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort_by_key(|&a_i| Reverse(a_i));

    let mut map = BTreeMap::new();
    for a_i in a.iter().copied() {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let mut count = 0;
    for a_i in a {
        match map.get_mut(&a_i) {
            None => unreachable!(),
            Some(count_a_i) => {
                if *count_a_i == 0 {
                    continue;
                }
                *count_a_i -= 1;
            }
        }

        let sum = (a_i + if a_i.is_power_of_two() { 1 } else { 0 }).next_power_of_two();
        let a_j = sum - a_i;
        match map.get_mut(&a_j) {
            None => continue,
            Some(count_a_j) => {
                if *count_a_j == 0 {
                    continue;
                }
                *count_a_j -= 1;
                count += 1;
            }
        }
    }

    let ans = count;
    println!("{}", ans);
}
