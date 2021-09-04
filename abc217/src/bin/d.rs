use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q],
    };
    let mut v = BTreeSet::new();
    v.insert(0);
    v.insert(l);
    for (c_i, x_i) in cx {
        let r = *v.range(x_i + 1..=l).next().unwrap();
        let l = *v.range(0..=x_i).next_back().unwrap();
        match c_i {
            1 => {
                v.insert(x_i);
            }
            2 => {
                println!("{}", r - l);
            }
            _ => unreachable!(),
        }
    }
}
