use std::collections::BTreeSet;

use proconio::input;

fn f(set: &mut BTreeSet<usize>, len: usize, ds: &mut Vec<usize>) {
    if len == 0 {
        let mut v = 0_usize;
        for d in ds.iter().copied() {
            v = v * 10 + d;
        }
        set.insert(v);
        return;
    }
    for i in (0..*ds.last().unwrap_or(&10)).rev() {
        ds.push(i);
        f(set, len - 1, ds);
        ds.pop();
    }
}

fn main() {
    input! {
        k: usize,
    };

    let mut set = BTreeSet::new();
    for len in 1..=10 {
        f(&mut set, len, &mut vec![]);
    }
    println!("{}", set.iter().nth(k).unwrap());
}
