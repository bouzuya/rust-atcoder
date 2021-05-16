use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [[Usize1; n - 1]; n],
    };
    let mut a = {
        let mut b = vec![];
        for a_i in a {
            b.push(a_i.into_iter().collect::<VecDeque<usize>>());
        }
        b
    };
    for d in 0.. {
        let mut next = false;
        let mut done = true;
        let mut ms = vec![n; n];

        for i in 0..n {
            if a[i].is_empty() {
                continue;
            }
            done = false;
            let p = a[i][0];
            if a[p][0] == i {
                ms[i] = p;
                ms[p] = i;
                next = true;
            }
        }
        if done {
            println!("{}", d);
            break;
        } else if !next {
            println!("-1");
            break;
        } else {
            for i in 0..n {
                if ms[i] == n {
                    continue;
                }
                a[ms[i]].pop_front();
            }
        }
    }
}
