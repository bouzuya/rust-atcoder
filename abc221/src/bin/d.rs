use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let mut set = BTreeSet::new();
    for (a_i, b_i) in ab.iter().copied() {
        set.insert(a_i);
        set.insert(a_i + b_i);
    }

    let mut map = BTreeMap::new();
    let mut map_rev = vec![0_usize; set.len()];
    for (index_of_day, day) in set.iter().copied().enumerate() {
        map.insert(day, index_of_day);
        map_rev[index_of_day] = day;
    }

    let mut count_each_day = vec![0_i64; set.len() + 1];
    for (a_i, b_i) in ab.iter().copied() {
        count_each_day[*map.get(&a_i).unwrap()] += 1;
        count_each_day[*map.get(&(a_i + b_i)).unwrap()] -= 1;
    }
    for i in 0..count_each_day.len() - 1 {
        count_each_day[i + 1] += count_each_day[i];
    }

    let mut count = vec![0; n + 1];
    for (day, c) in count_each_day.iter().copied().enumerate() {
        if c == 0 {
            continue;
        }

        let days = map_rev[day + 1] - map_rev[day];
        count[c as usize] += days;
    }

    for k in 1..=n {
        println!("{}", count[k]);
    }
}
