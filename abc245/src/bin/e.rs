use std::{cmp::Reverse, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; m],
        d: [usize; m],
    };
    #[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    enum Type {
        Box,
        Chocolate,
    }
    let ab = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| (a, b, Type::Chocolate));
    let cd = c
        .into_iter()
        .zip(d.into_iter())
        .map(|(c, d)| (c, d, Type::Box));
    let mut abcd = ab.chain(cd).collect::<Vec<(usize, usize, Type)>>();
    abcd.sort_by_key(|&(ac, bd, t)| (Reverse(ac), t, bd));
    let mut boxes = BTreeMap::new();
    for (ac, bd, t) in abcd {
        match t {
            Type::Box => {
                let (_, d) = (ac, bd);
                *boxes.entry(d).or_insert(0) += 1;
            }
            Type::Chocolate => {
                let (_, b) = (ac, bd);
                let mut remove = None;
                if let Some((&d, count)) = boxes.range_mut(b..).next() {
                    *count -= 1;
                    if *count == 0 {
                        remove = Some(d);
                    }
                } else {
                    println!("No");
                    return;
                }
                if let Some(d) = remove {
                    boxes.remove(&d);
                }
            }
        }
    }

    println!("Yes");
}
