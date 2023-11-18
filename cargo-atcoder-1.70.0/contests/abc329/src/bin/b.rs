use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let set = a.into_iter().collect::<BTreeSet<usize>>();
    let mut it = set.iter().rev();
    it.next();
    let ans = it.next().unwrap();
    println!("{}", ans);
}
