use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut count = HashMap::new();
    let mut numbers = a.clone();
    numbers.sort();
    for (i, number) in numbers.into_iter().enumerate() {
        count.insert(number, k / n + if i < k % n { 1 } else { 0 });
    }
    for a_i in a {
        println!("{}", count.get(&a_i).unwrap());
    }
}
