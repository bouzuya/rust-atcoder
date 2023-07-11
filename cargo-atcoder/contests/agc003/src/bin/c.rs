use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    let mut set1 = BTreeSet::new();
    for (i, &a_i) in a.iter().enumerate() {
        if i % 2 == 0 {
            set1.insert(a_i);
        }
    }

    a.sort();

    let mut set2 = BTreeSet::new();
    for (i, &a_i) in a.iter().enumerate() {
        if i % 2 != 0 {
            set2.insert(a_i);
        }
    }

    let ans = set1.iter().filter(|a_i| set2.contains(&a_i)).count();
    println!("{}", ans);
}
