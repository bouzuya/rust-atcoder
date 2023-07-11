// WA
use std::{cmp::min, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert(i);
    }
    let mut counts = vec![0; n];
    for i in 0..m {
        counts[a[i]] += 1;
        set.remove(&a[i]);
    }
    let mut min_x = *set.iter().next().unwrap_or(&n);
    for i in m..n {
        counts[a[i]] += 1;
        set.remove(&a[i]);
        counts[a[i - m]] -= 1;
        if counts[a[i - m]] == 0 {
            set.insert(a[i - m]);
        }
        min_x = min(min_x, *set.iter().next().unwrap_or(&n));
    }
    let ans = min_x;
    println!("{}", ans);
}
