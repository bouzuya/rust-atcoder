use std::collections::{BTreeSet, HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let mut map = HashMap::new();
    map.insert(1, Vec::new());
    for (a, b) in ab.iter().copied() {
        map.entry(a).or_insert_with(Vec::new).push(b);
        map.entry(b).or_insert_with(Vec::new).push(a);
    }
    let mut dist = BTreeSet::new();
    let mut deque = VecDeque::new();
    dist.insert(1);
    deque.push_back(1);
    while let Some(u) = deque.pop_front() {
        for v in map.get(&u).unwrap().iter().copied() {
            if dist.insert(v) {
                deque.push_back(v);
            }
        }
    }
    println!("{}", dist.last().unwrap());
}
