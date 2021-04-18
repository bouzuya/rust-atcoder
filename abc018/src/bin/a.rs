use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        abc: [i64; 3],
    };
    let mut values = abc
        .iter()
        .enumerate()
        .map(|(abc, &v)| (v, abc))
        .collect::<Vec<(i64, usize)>>();
    values.sort_by_key(|&(v, _)| Reverse(v));
    let mut with_rank = values
        .iter()
        .enumerate()
        .map(|(i, &(_, abc))| (abc, i + 1))
        .collect::<Vec<(usize, usize)>>();
    with_rank.sort_by_key(|&(abc, _)| abc);
    for (_, r) in with_rank {
        println!("{}", r);
    }
}
