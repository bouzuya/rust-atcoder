use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = BTreeMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        map.entry(a_i).or_insert_with(Vec::new).push(i);
    }

    let mut b = vec![n; n];
    let mut index = 0_usize;
    for (_, is) in map {
        for i in is {
            b[i] = index;
            index += 1;
        }
    }

    for b_i in b {
        print!("{} ", b_i + 1);
    }
}
