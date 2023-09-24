use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
    }

    let mut a = vec![];
    for _ in 0..m {
        input! {
            k_i: usize,
            a_i: [usize; k_i],
        }
        a.push(a_i);
    }

    let mut map = HashMap::new();
    for (i, a_i) in a.iter().enumerate() {
        let a_ir = *a_i.last().unwrap();
        map.entry(a_ir).or_insert_with(Vec::new).push(i);
    }

    let mut q = VecDeque::new();
    for (k, v) in map.iter() {
        if v.len() == 2 {
            q.push_back((*k, v[0], v[1]));
        }
    }

    while let Some((_, i, j)) = q.pop_front() {
        a[i].pop();
        a[j].pop();
        if let Some(&x1) = a[i].last() {
            map.entry(x1).or_insert_with(Vec::new).push(i);
            if let Some(v) = map.get(&x1) {
                if v.len() == 2 {
                    q.push_back((x1, v[0], v[1]));
                }
            }
        }
        if let Some(&x2) = a[j].last() {
            map.entry(x2).or_insert_with(Vec::new).push(j);
            if let Some(v) = map.get(&x2) {
                if v.len() == 2 {
                    q.push_back((x2, v[0], v[1]));
                }
            }
        }
    }

    let ans = a.iter().all(|a_i| a_i.is_empty());
    println!("{}", if ans { "Yes" } else { "No" });
}
