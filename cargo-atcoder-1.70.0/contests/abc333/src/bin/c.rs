use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: Usize1,
    };

    let mut rep = vec![];
    let mut x = 1;
    rep.push(x);
    for _ in 0..14 {
        x *= 10;
        x += 1;
        rep.push(x);
    }

    let mut set = HashSet::new();
    for i in rep.iter().copied() {
        for j in rep.iter().copied() {
            for k in rep.iter().copied() {
                set.insert(i + j + k);
            }
        }
    }

    let mut set = set.iter().copied().collect::<Vec<usize>>();
    set.sort();
    let ans = set[n];
    println!("{}", ans);
}
