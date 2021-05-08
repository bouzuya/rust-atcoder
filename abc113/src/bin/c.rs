use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        py: [(u64, u64); m],
    };
    let mut ipy = py
        .iter()
        .enumerate()
        .map(|(i, &(p_i, y_i))| (i, p_i, y_i))
        .collect::<Vec<(usize, u64, u64)>>();
    ipy.sort_by_key(|&(_, p, y)| (p, y));
    let mut count = BTreeMap::new();
    let mut is = ipy
        .iter()
        .map(|&(i, p, _)| {
            *count.entry(p).or_insert(0) += 1;
            (i, format!("{:06}{:06}", p, count.get(&p).unwrap()))
        })
        .collect::<Vec<(usize, String)>>();
    is.sort_by_key(|&(i, _)| i);
    for (_, s) in is {
        println!("{}", s);
    }
}
