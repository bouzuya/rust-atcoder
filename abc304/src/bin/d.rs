use std::collections::HashMap;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        _w: usize,
        _h: usize,
        n: usize,
        pq: [(usize, usize); n],
        capital_a: usize,
        a: [usize; capital_a],
        capital_b: usize,
        b: [usize; capital_b],
    };
    let a = {
        let mut a2 = vec![0];
        a2.extend(a);
        a2
    };
    let b = {
        let mut b2 = vec![0];
        b2.extend(b);
        b2
    };

    let min = if n < (capital_a + 1) * (capital_b + 1) {
        0
    } else {
        let mut count = vec![vec![0; capital_b + 1]; capital_a + 1];
        for (p, q) in pq.iter().copied() {
            let i = a.lower_bound(&p) - 1;
            let j = b.lower_bound(&q) - 1;
            count[i][j] += 1;
        }

        let mut min = n;
        for i in 0..capital_a + 1 {
            for j in 0..capital_b + 1 {
                min = min.min(count[i][j]);
            }
        }
        min
    };

    let max = {
        let mut count = HashMap::new();
        for (p, q) in pq.iter().copied() {
            let i = a.lower_bound(&p) - 1;
            let j = b.lower_bound(&q) - 1;
            *count.entry((i, j)).or_insert(0_usize) += 1;
        }
        *count.values().max().unwrap()
    };

    println!("{} {}", min, max);
}
