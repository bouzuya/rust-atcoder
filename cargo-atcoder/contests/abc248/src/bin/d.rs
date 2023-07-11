use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [(usize, usize, usize); q],
    };

    let mut in_map = BTreeMap::new();
    let mut out_map = BTreeMap::new();
    for (i, (l, r, x)) in lrx.iter().copied().enumerate() {
        in_map.entry(l - 1).or_insert_with(Vec::new).push((x, i));
        out_map.entry(r - 1).or_insert_with(Vec::new).push((x, i));
    }

    let mut ans = vec![(0, 0); q];
    let mut count = BTreeMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        match in_map.get(&i) {
            None => {}
            Some(ios) => {
                for (x, j) in ios.iter().copied() {
                    ans[j].0 = *count.get(&x).unwrap_or(&0);
                }
            }
        }
        *count.entry(a_i).or_insert(0) += 1;
        match out_map.get(&i) {
            None => {}
            Some(ios) => {
                for (x, j) in ios.iter().copied() {
                    ans[j].1 = *count.get(&x).unwrap_or(&0);
                }
            }
        }
    }
    for (i, o) in ans {
        println!("{}", o - i);
    }
}
