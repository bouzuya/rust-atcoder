use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut ab: [(Usize1, usize); n]
    };

    ab.sort();
    let mut index = 0_usize;
    let mut map = BTreeMap::new();
    let mut sum = 0_usize;
    for k in 0..n {
        while index < ab.len() && ab[index].0 <= k {
            *map.entry(ab[index].1).or_insert(0) += 1;
            index += 1;
        }

        let max = *map.keys().next_back().unwrap();
        let count = map.get_mut(&max).unwrap();
        *count -= 1;
        if *count == 0 {
            map.remove(&max);
        }
        sum += max;
        println!("{}", sum);
    }
}
