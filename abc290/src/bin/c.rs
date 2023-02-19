use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let set = a
        .iter()
        .copied()
        .filter(|&a_i| a_i <= k)
        .collect::<BTreeSet<_>>();
    for i in 0..k {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }
    println!("{}", k);
}
