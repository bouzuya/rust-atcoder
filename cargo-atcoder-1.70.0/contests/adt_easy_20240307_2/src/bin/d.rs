use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let set = a.iter().copied().collect::<BTreeSet<usize>>();
    for x in 0..=n + 1 {
        if set.contains(&x) {
            continue;
        }
        println!("{}", x);
        return;
    }
}
