use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };
    let mut map_a = HashMap::new();
    for a_i in a {
        *map_a.entry(a_i).or_insert(0) += 1;
    }
    let mut map_b = HashMap::new();
    for b_i in b {
        *map_b.entry(b_i).or_insert(0) += 1;
    }

    for (k, v) in map_b {
        let ok = match map_a.get(&k) {
            None => false,
            Some(&count) => count >= v,
        };
        if !ok {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
