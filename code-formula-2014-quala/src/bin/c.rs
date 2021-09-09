use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; k]; n]
    };

    let mut set = BTreeSet::new();
    let mut heap = BinaryHeap::new();
    for (i, a_i) in a.iter().enumerate() {
        for (j, a_ij) in a_i.iter().copied().enumerate() {
            heap.push(Reverse((j, i, a_ij)));
        }

        let mut ans = vec![];
        while let Some(Reverse((x_j, x_i, x_a_ij))) = heap.pop() {
            if x_j * (n - 1 - i) + set.len() < k {
                if set.insert(x_a_ij) {
                    ans.push(x_a_ij);
                }
            } else {
                heap.push(Reverse((x_j, x_i, x_a_ij)));
                break;
            }
        }
        ans.sort();
        println!(
            "{}",
            ans.into_iter()
                .map(|a_ij| a_ij.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
