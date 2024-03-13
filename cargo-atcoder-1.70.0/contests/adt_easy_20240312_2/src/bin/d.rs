use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 4 * n - 1],
    };
    let mut map = HashMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0_usize) += 1;
    }
    for x in 1..=n {
        let count = *map.get(&x).unwrap_or(&0);
        if count != 4 {
            println!("{}", x);
            return;
        }
    }
}
