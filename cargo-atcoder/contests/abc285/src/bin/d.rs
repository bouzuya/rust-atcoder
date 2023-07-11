use std::collections::{BTreeMap, BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    let mut set = BTreeSet::new();
    for (s, t) in st.iter() {
        set.insert(s.to_string());
        set.insert(t.to_string());
    }
    let mut map = BTreeMap::new();
    for (i, k) in set.iter().cloned().enumerate() {
        map.insert(k, i);
    }

    let m = map.len();
    let mut edges = vec![m; m];
    for (s, t) in st.iter() {
        let u = *map.get(s).unwrap();
        let v = *map.get(t).unwrap();
        edges[v] = u;
    }

    let mut used = vec![false; m];
    for (s, _) in st.iter() {
        let u = *map.get(s).unwrap();
        used[u] = true;
    }
    let mut queue = VecDeque::new();
    for (_, t) in st.iter() {
        let v = *map.get(t).unwrap();
        if !used[v] {
            queue.push_back(v);
        }
    }

    let mut count = 0_usize;
    while let Some(v) = queue.pop_front() {
        // u -> v
        let u = edges[v];
        if u != m {
            count += 1;
            queue.push_back(u);
        }
    }

    let ans = count == n;
    println!("{}", if ans { "Yes" } else { "No" });
}
