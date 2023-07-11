use std::{cmp, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [usize; n],
    };

    let inf = 1_000_000_000;
    let mut map = BTreeMap::new();
    map.insert(0, 0);
    for i in 0..n {
        let t_i = t[i];
        let mut next = BTreeMap::new();
        for (v, a) in map {
            next.insert(v, a + t_i);
            next.insert(v + t_i, a);
        }
        map = next;
    }
    let mut ans = inf;
    for (v, a) in map {
        ans = cmp::min(ans, cmp::max(v, a));
    }
    println!("{}", ans);

    // WA
    // t.sort_by_key(|&t_i| cmp::Reverse(t_i));

    // let mut t1 = 0_i64;
    // let mut t2 = 0_i64;
    // for &t_i in t.iter() {
    //     if t1 <= t2 {
    //         t1 += t_i;
    //     } else {
    //         t2 += t_i;
    //     }
    // }

    // let ans = cmp::max(t1, t2);
    // println!("{}", ans);
}
