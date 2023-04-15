use std::collections::{BTreeMap, BTreeSet, HashMap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut boxes = vec![BTreeMap::new(); n];
    let mut cards = HashMap::new();
    for _ in 0..q {
        input! {
            t: usize,
            i: Usize1,
        }
        match t {
            1 => {
                input! {
                    j: Usize1,
                }
                *boxes[j].entry(i).or_insert(0) += 1;
                cards.entry(i).or_insert_with(BTreeSet::new).insert(j);
            }
            2 => {
                for (x, &count) in boxes[i].iter() {
                    for _ in 0..count {
                        println!("{}", x + 1);
                    }
                }
            }
            3 => {
                for x in cards.get(&i).unwrap_or(&BTreeSet::new()).iter().copied() {
                    println!("{}", x + 1);
                }
            }
            _ => unreachable!(),
        }
    }
}
