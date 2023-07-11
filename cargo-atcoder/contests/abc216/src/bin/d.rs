use std::collections::{BTreeMap, BTreeSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut is = vec![vec![]; n];
    let mut x = vec![];
    for i in 0..m {
        input! {
            k_i: usize,
            a_i: [Usize1; k_i],
        }
        let mut deque = VecDeque::new();
        for a_ij in a_i {
            is[a_ij].push(i);
            deque.push_back(a_ij);
        }
        x.push(deque);
    }

    let mut set = BTreeSet::new();
    let mut map = BTreeMap::new();
    for x_i in x.iter() {
        let x_top = *x_i.front().unwrap();
        let entry = map.entry(x_top).or_insert(0);
        *entry += 1;
        if *entry == 2 {
            set.insert(x_top);
        }
    }

    while !set.is_empty() {
        let c = *set.iter().next().unwrap();
        set.remove(&c);
        map.remove(&c);
        let i_0 = is[c][0];
        let i_1 = is[c][1];

        x[i_0].pop_front();
        if let Some(&c_0) = x[i_0].front() {
            let entry = map.entry(c_0).or_insert(0);
            *entry += 1;
            if *entry == 2 {
                set.insert(c_0);
            }
        }

        x[i_1].pop_front();
        if let Some(&c_1) = x[i_1].front() {
            let entry = map.entry(c_1).or_insert(0);
            *entry += 1;
            if *entry == 2 {
                set.insert(c_1);
            }
        }
    }

    let ans = map.is_empty();
    println!("{}", if ans { "Yes" } else { "No" });
}
