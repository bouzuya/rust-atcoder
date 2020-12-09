use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    };
    let mut map_count = BTreeMap::new();
    for s_i in s {
        *map_count.entry(s_i).or_insert(0) += 1;
    }
    let mut map_rank = BTreeMap::new();
    for (s_i, count) in map_count {
        map_rank.entry(count).or_insert(vec![]).push(s_i);
    }
    let mut count = 0;
    for (_, values) in map_rank.iter().rev() {
        count += values.len();
        if count >= k {
            if values.len() == 1 {
                println!("{}", values[0]);
            } else {
                println!("AMBIGUOUS");
            }
            return;
        }
    }
}
