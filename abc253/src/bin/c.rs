use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut s = BTreeMap::new();
    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {
            t: usize,
        };
        match t {
            1 => {
                input! {
                    x: usize,
                }
                *s.entry(x).or_insert(0_usize) += 1;
                set.insert(x);
            }
            2 => {
                input! {
                    x: usize,
                    c: usize,
                }
                let entry = s.entry(x).or_insert(0);
                *entry = (*entry).saturating_sub(c);
                if *entry == 0 {
                    s.remove(&x);
                    set.remove(&x);
                }
            }
            3 => {
                let min = *set.iter().next().unwrap();
                let max = *set.iter().rev().next().unwrap();
                println!("{}", max - min);
            }
            _ => unreachable!(),
        }
    }
}
