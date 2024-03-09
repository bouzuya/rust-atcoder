use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        n: usize,
    };

    let mut s = vec![];
    for _ in 0..n {
        input! {
            a_i: usize,
            s_i: [Chars; a_i],
        }
        s.push(s_i);
    }

    let mut map = BTreeMap::new();
    map.insert(0_usize, 0_usize);
    for s_i in s {
        let mut next = map.clone();
        for (x, min_x) in map {
            for s_ij in s_i.iter() {
                if x + s_ij.len() > t.len() {
                    continue;
                }
                if s_ij == &t[x..x + s_ij.len()] {
                    let entry = next.entry(x + s_ij.len()).or_insert(min_x + 1);
                    *entry = (*entry).min(min_x + 1);
                }
            }
        }
        map = next;
    }
    match map.get(&t.len()) {
        Some(x) => println!("{}", x),
        None => println!("-1"),
    }
}
