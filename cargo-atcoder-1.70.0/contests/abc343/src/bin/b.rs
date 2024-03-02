use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    };
    let mut edges = vec![BTreeSet::new(); n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                edges[i].insert(j);
            }
        }
    }
    for e in edges {
        let len = e.len();
        for (i, v) in e.into_iter().enumerate() {
            print!("{}{}", v + 1, if i == len - 1 { '\n' } else { ' ' });
        }
    }
}
