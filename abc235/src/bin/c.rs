use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, usize); q],
    };

    let mut map = HashMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        map.entry(a_i).or_insert(vec![]).push(i);
    }

    for (x, k) in xk {
        match map.get(&x) {
            None => {
                println!("-1");
                continue;
            }
            Some(xs) => {
                if k - 1 >= xs.len() {
                    println!("-1");
                    continue;
                }
                println!("{}", xs[k - 1] + 1);
            }
        }
    }
}
