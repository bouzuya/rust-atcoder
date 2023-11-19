use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [Usize1; n],
        ab: [(Usize1, Usize1); q],
    };

    let mut boxes = vec![HashSet::new(); n];
    for (i, c_i) in c.iter().copied().enumerate() {
        boxes[i].insert(c_i);
    }

    for (a, b) in ab {
        let (small, large) = if boxes[a].len() < boxes[b].len() {
            (a, b)
        } else {
            (b, a)
        };
        boxes.push(HashSet::new());
        boxes.swap(small, n);
        for x in boxes.pop().unwrap() {
            boxes[large].insert(x);
        }
        if large != b {
            boxes.swap(small, large);
        }
        println!("{}", boxes[b].len());
    }
}
