use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
        b: [usize; m],
        cd: [(Usize1, Usize1); l],
    };

    let mut bi = b
        .iter()
        .enumerate()
        .map(|(i, b_i)| (*b_i, i))
        .collect::<Vec<(usize, usize)>>();
    bi.sort_by_key(|(b_i, _)| *b_i);
    bi.reverse();

    let mut sets = HashMap::new();
    for (c, d) in cd {
        sets.entry(c).or_insert_with(HashSet::new).insert(d);
    }

    let mut max = 0_usize;
    for (i, a_i) in a.iter().enumerate() {
        match sets.get(&i) {
            Some(set) => {
                for (b_i, i) in bi.iter().copied() {
                    if !set.contains(&i) {
                        max = max.max(a_i + b_i);
                        break;
                    }
                }
            }
            None => max = max.max(a_i + bi[0].0),
        }
    }

    let ans = max;
    println!("{}", ans);
}
