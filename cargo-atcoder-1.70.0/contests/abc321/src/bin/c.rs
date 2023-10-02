use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    let mut set = BTreeSet::new();
    let ds = "9876543210".chars().collect::<Vec<char>>();
    for bits in 1..1 << ds.len() {
        let x = (0..ds.len())
            .filter(|i| ((bits >> i) & 1) == 1)
            .map(|i| ds[i])
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        set.insert(x);
    }
    set.remove(&0);
    let ans = *set.iter().nth(k - 1).unwrap();
    println!("{}", ans);
}
