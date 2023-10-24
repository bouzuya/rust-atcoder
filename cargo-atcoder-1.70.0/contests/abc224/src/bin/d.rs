use std::collections::{HashMap, HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        p: [Usize1; 8]
    };
    let p = {
        let set = p.iter().copied().collect::<HashSet<usize>>();
        let none = (0..9).find(|i| !set.contains(i)).unwrap();
        let mut v = vec![10; 9];
        for (i, p_i) in p.into_iter().chain(std::iter::once(none)).enumerate() {
            v[p_i] = i;
        }
        v
    };
    let mut map = HashMap::new();
    let mut deque = VecDeque::new();
    map.insert(p.clone(), 0_i64);
    deque.push_back(p);
    while let Some(p) = deque.pop_front() {
        let count = map[&p];
        for (u, v) in uv.iter().copied() {
            if p[u] != 8 && p[v] != 8 {
                continue;
            }
            let mut p = p.clone();
            p.swap(u, v);
            if map.contains_key(&p) {
                continue;
            }
            map.insert(p.clone(), count + 1);
            deque.push_back(p);
        }
    }

    let ans = map.get(&(0..9).collect::<Vec<usize>>()).unwrap_or(&-1);
    println!("{}", ans);
}
