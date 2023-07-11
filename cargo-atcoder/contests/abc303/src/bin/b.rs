use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1; n]; m],
    };

    let mut set = HashSet::new();
    for i in 0..m {
        for j in 1..n {
            let l = a[i][j - 1];
            let r = a[i][j];
            set.insert((l.min(r), l.max(r)));
        }
    }

    let mut count = 0_usize;
    for i in 0..n {
        for j in i + 1..n {
            if !set.contains(&(i, j)) {
                count += 1;
            }
        }
    }

    let ans = count;
    println!("{}", ans);
}
