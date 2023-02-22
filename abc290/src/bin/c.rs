use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let set = a.into_iter().collect::<HashSet<_>>();
    for i in 0..=k {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }
    println!("{}", k);
}
