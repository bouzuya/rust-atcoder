use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut map_s = BTreeMap::new();
    for s_i in s.iter().copied() {
        *map_s.entry(s_i).or_insert(0) += 1;
    }
    let mut map_t = BTreeMap::new();
    for t_i in t.iter().copied() {
        *map_t.entry(t_i).or_insert(0) += 1;
    }
    // println!("{:?}", map_s);
    // println!("{:?}", map_t);
    for (k, v1) in map_s.iter_mut() {
        if k == &'@' {
            continue;
        }
        if let Some(v2) = map_t.get_mut(k) {
            let d = (*v1).min(*v2);
            *v1 -= d;
            *v2 -= d;
        }
    }

    // println!("{:?}", map_s);
    // println!("{:?}", map_t);

    let mut count_s = 0_usize;
    for (k, v) in map_s.iter() {
        if v == &0 {
            continue;
        }
        if k == &'@' {
            continue;
        }
        if "atcoder".chars().all(|c| c != *k) {
            println!("No");
            return;
        }
        count_s += v;
    }

    let mut count_t = 0_usize;
    for (k, v) in map_t.iter() {
        if v == &0 {
            continue;
        }
        if k == &'@' {
            continue;
        }
        if "atcoder".chars().all(|c| c != *k) {
            println!("No");
            return;
        }
        count_t += v;
    }

    // println!("{:?}", count_s);
    // println!("{:?}", count_t);

    let ans =
        *map_s.get(&'@').unwrap_or(&0) >= count_t && *map_t.get(&'@').unwrap_or(&0) >= count_s;
    println!("{}", if ans { "Yes" } else { "No" });
}
