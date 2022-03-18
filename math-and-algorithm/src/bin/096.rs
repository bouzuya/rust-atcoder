use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    };
    let sum = t.iter().sum::<usize>();
    let mut set = BTreeSet::new();
    set.insert(0);
    for t_i in t {
        let mut next = set.clone();
        for x in set.iter().copied() {
            next.insert(x + t_i);
        }
        set = next;
    }
    let ans = set.iter().copied().map(|x| x.max(sum - x)).min().unwrap();
    println!("{}", ans);
}
