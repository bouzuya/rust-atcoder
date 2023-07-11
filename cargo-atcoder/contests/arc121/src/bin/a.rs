use std::{cmp, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut x = xy
        .iter()
        .enumerate()
        .map(|(i, &(x_i, _))| (i, x_i))
        .collect::<Vec<(usize, i64)>>();
    let mut y = xy
        .iter()
        .enumerate()
        .map(|(i, &(_, y_i))| (i, y_i))
        .collect::<Vec<(usize, i64)>>();

    x.sort_by_key(|&(_, x_i)| x_i);
    y.sort_by_key(|&(_, x_i)| x_i);
    let mut a = vec![
        (x[0].0, x[n - 1].0, (x[0].1 - x[n - 1].1).abs()),
        (x[0].0, x[n - 2].0, (x[0].1 - x[n - 2].1).abs()),
        (x[1].0, x[n - 1].0, (x[1].1 - x[n - 1].1).abs()),
        (x[1].0, x[n - 2].0, (x[1].1 - x[n - 2].1).abs()),
        (y[0].0, y[n - 1].0, (y[0].1 - y[n - 1].1).abs()),
        (y[0].0, y[n - 2].0, (y[0].1 - y[n - 2].1).abs()),
        (y[1].0, y[n - 1].0, (y[1].1 - y[n - 1].1).abs()),
        (y[1].0, y[n - 2].0, (y[1].1 - y[n - 2].1).abs()),
    ];
    a.sort_by_key(|&(_, _, v)| -v);
    let mut first = true;
    let mut set = BTreeSet::new();
    for (i, j, x) in a {
        if set.insert((cmp::min(i, j), cmp::max(i, j))) {
            if first {
                first = false;
                continue;
            }
            println!("{}", x);
            return;
        }
    }
}
