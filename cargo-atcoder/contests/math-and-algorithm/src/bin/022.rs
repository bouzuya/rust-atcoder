use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = HashMap::new();
    for a_i in a {
        *count.entry(a_i).or_insert(0) += 1;
    }
    let mut sum = 0_usize;
    for (&x, &count_x) in count.iter() {
        let y = 100_000 - x;
        if let Some(&count_y) = count.get(&y) {
            sum += count_x * (count_y - if x == y { 1 } else { 0 });
        }
    }
    let ans = sum / 2;
    println!("{}", ans);
}
