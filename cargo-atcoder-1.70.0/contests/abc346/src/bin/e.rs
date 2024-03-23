use std::collections::{BTreeMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        tax: [(usize, Usize1, usize); m],
    };

    let mut set_h = HashSet::new();
    let mut set_w = HashSet::new();
    let mut colors = BTreeMap::new();
    for (t, a, x) in tax.into_iter().rev() {
        match t {
            1 => {
                if set_h.insert(a) {
                    let count = w - set_w.len();
                    if count > 0 {
                        *colors.entry(x).or_insert(0) += count;
                    }
                }
            }
            2 => {
                if set_w.insert(a) {
                    let count = h - set_h.len();
                    if count > 0 {
                        *colors.entry(x).or_insert(0) += count;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    let count = h * w - (set_h.len() * w + set_w.len() * h - set_h.len() * set_w.len());
    if count > 0 {
        *colors.entry(0).or_insert(0) += count;
    }
    println!("{}", colors.len());
    for (c, v) in colors {
        println!("{} {}", c, v);
    }
}
