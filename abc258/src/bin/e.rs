use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        x: usize,
        w: [usize; n],
        k: [Usize1; q],
    };

    let sum_w = w.iter().sum::<usize>();
    let w = w.repeat(2);
    let b = (x / sum_w) * n;
    let x = x % sum_w;
    let mut next = vec![0; n];
    let mut sum = 0;
    let mut j = 0;
    for i in 0..n {
        while sum < x {
            sum += w[j];
            j += 1;
        }
        next[i] = j;
        sum -= w[i];
    }

    let mut boxes = vec![];
    let mut index = 0;
    let mut map = HashMap::new();
    let (head, cycle) = loop {
        match map.insert(index, boxes.len()) {
            Some(prev) => {
                break boxes.split_at(prev);
            }
            None => {
                boxes.push(next[index] - index);
                index = next[index] % n;
            }
        }
    };

    for k_i in k {
        let ans = if k_i < head.len() {
            head[k_i]
        } else {
            cycle[(k_i - head.len()) % cycle.len()]
        } + b;
        println!("{}", ans);
    }
}
