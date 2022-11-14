use std::collections::{BTreeMap, BTreeSet, HashMap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let mut map = HashMap::new();
    for i in 0..n - 1 {
        let t = format!("{}{}", s[i], s[i + 1]);
        *map.entry(t).or_insert(0_usize) += 1;
    }
    let mut rev = BTreeMap::new();
    for (t, count) in map {
        rev.entry(count).or_insert_with(BTreeSet::new).insert(t);
    }
    let ans = rev.iter().last().unwrap().1.iter().next().unwrap();
    println!("{}", ans);
}
