use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    };

    let mut map = HashMap::new();
    for s_i in s {
        *map.entry(s_i).or_insert(0_usize) += 1;
    }

    let mut map2 = BTreeMap::new();
    for (s, c) in map {
        map2.entry(Reverse(c)).or_insert_with(Vec::new).push(s);
    }

    let mut sum = 0_usize;
    for (_, ss) in map2.into_iter() {
        if sum + ss.len() < k {
            sum += ss.len();
            continue;
        }
        if ss.len() == 1 {
            println!("{}", ss[0]);
        } else {
            println!("AMBIGUOUS");
        }
        return;
    }
}
