use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); n],
    };

    // 0: red, 1: blue
    let mut pt = ab
        .into_iter()
        .map(|(a, b)| (a, b, 0))
        .chain(cd.into_iter().map(|(c, d)| (c, d, 1)))
        .collect::<Vec<(usize, usize, usize)>>();
    pt.sort_by_key(|&(x, y, c)| (x, c, y));

    let mut count = 0_usize;
    let mut ry = BTreeSet::new();
    for (_, y, c) in pt {
        match c {
            0 => {
                ry.insert(y);
            }
            1 => {
                if let Some(&ty) = ry.range(0..y).rev().next() {
                    count += 1;
                    ry.remove(&ty);
                }
            }
            _ => unreachable!(),
        }
    }

    let ans = count;
    println!("{}", ans);
}
