use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    for a_i in a.iter().copied() {
        *map.entry(a_i).or_insert(0_usize) += 1;
    }

    let mut sum = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        let count_a_i = map.get_mut(&a_i).unwrap();
        *count_a_i -= 1;
        let count = n - 1 - i - map[&a_i];
        sum += count;
    }

    let ans = sum;
    println!("{}", ans);
}
