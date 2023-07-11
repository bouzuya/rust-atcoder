use std::collections::BTreeSet;

use proconio::input;

fn dfs(set: &mut BTreeSet<String>, s: &mut Vec<char>, n: usize) {
    if s.len() == n {
        set.insert(s.iter().collect::<String>());
        return;
    }
    for c in "abc".chars() {
        s.push(c);
        dfs(set, s, n);
        s.pop();
    }
}

fn main() {
    input! {
        n: usize,
    };
    let mut set = BTreeSet::new();
    let mut buf = vec![];
    dfs(&mut set, &mut buf, n);
    for s in set {
        println!("{}", s);
    }
}
