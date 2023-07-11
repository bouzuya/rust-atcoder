use std::{cmp, collections::HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    };
    let mut sum_a = 0;
    let mut sum_b = 0;
    for (a, b) in ab.clone() {
        sum_a += a;
        sum_b += b;
        if a >= x && b >= y {
            println!("1");
            return;
        }
    }
    if sum_a < x || sum_b < y {
        println!("-1");
        return;
    }

    let mut map = HashMap::new();
    for (a_i, b_i) in ab {
        let mut next = map.clone();
        for ((a_x, b_x), count) in map {
            let a = cmp::min(a_x + a_i, x);
            let b = cmp::min(b_x + b_i, y);
            let entry = next.entry((a, b)).or_insert(count + 1);
            *entry = cmp::min(*entry, count + 1);
        }
        let entry = next
            .entry((cmp::min(a_i, x), cmp::min(b_i, y)))
            .or_insert(1_usize);
        *entry = cmp::min(*entry, 1);

        map = next;
    }
    let ans = *map.get(&(x, y)).unwrap();
    println!("{}", ans);
}
