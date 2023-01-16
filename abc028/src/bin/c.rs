use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        abcde: [usize; 5],
    };
    let mut set = BTreeSet::new();
    for i in 0..5 {
        for j in i + 1..5 {
            for k in j + 1..5 {
                let v = abcde[i] + abcde[j] + abcde[k];
                set.insert(v);
            }
        }
    }

    let mut iter = set.iter().rev();
    iter.next();
    iter.next();
    let ans = *iter.next().unwrap();
    println!("{}", ans);
}
