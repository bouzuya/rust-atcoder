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
    if m == n {
        for &a_i in a.iter() {
            set.remove(&a_i);
        }
        println!("{}", *set.iter().next().unwrap());
        return;
    }
    let mut min_x = 1_000_000_000;
    let mut map = vec![0; n];
    let mut i_m = 0;
    for i in 0..n - m {
        while i_m < i + m {
            map[a[i_m]] += 1;
            set.remove(&a[i_m]);
            i_m += 1;
        }

        min_x = min(min_x, *set.iter().next().unwrap());

        map[a[i]] -= 1;
        if map[a[i]] == 0 {
            set.insert(a[i]);
        }
    }
    let ans = min_x;
    println!("{}", ans);
}
