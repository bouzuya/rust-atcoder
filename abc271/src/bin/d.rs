use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    };
    let mut vs = vec![HashMap::new(); n + 1];
    vs[1].entry(ab[0].0).or_insert_with(HashSet::new).insert(0);
    vs[1].entry(ab[0].1).or_insert_with(HashSet::new).insert(0);
    for (i, (a_i, b_i)) in ab.iter().copied().enumerate().skip(1) {
        let prev = vs[i].clone();
        for &u in prev.keys().clone() {
            if a_i + u <= s {
                vs[i + 1]
                    .entry(a_i + u)
                    .or_insert_with(HashSet::new)
                    .insert(u);
            }
            if b_i + u <= s {
                vs[i + 1]
                    .entry(b_i + u)
                    .or_insert_with(HashSet::new)
                    .insert(u);
            }
        }
    }

    let ok = vs[n].contains_key(&s);
    println!("{}", if ok { "Yes" } else { "No" });
    if ok {
        let mut ans = vec![];
        let mut cur = s;
        for i in (0..n).rev() {
            let prev = *vs[i + 1].get(&cur).unwrap().iter().next().unwrap();
            let d = cur - prev;
            let a = if ab[i].0 == d {
                'H'
            } else if ab[i].1 == d {
                'T'
            } else {
                unreachable!()
            };
            ans.push(a);
            cur = prev;
        }
        ans.reverse();
        println!("{}", ans.iter().collect::<String>());
    }
}
