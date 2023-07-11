use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
    };
    let mut ans: Option<BTreeSet<String>> = None;
    for _ in 0..n {
        input! {
            k: usize,
            s: [String; k],
        }
        let mut set = BTreeSet::new();
        for j in 0..k {
            set.insert(s[j].clone());
        }
        ans = Some(match ans {
            None => set,
            Some(prev) => prev.intersection(&set).cloned().collect(),
        });
    }
    println!("{}", ans.unwrap().len());
}
