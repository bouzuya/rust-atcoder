use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        ck: [(char, Usize1); k],
    };
    let mut set = HashSet::new();
    for i in 0..k {
        set.insert(i);
    }
    let mut b = vec![];
    for _ in 0..n {
        b.push(set.clone());
    }

    for (i, (c_i, k_i)) in ck.into_iter().enumerate() {
        b[k_i] = HashSet::new();
        b[k_i].insert(i);
        match c_i {
            'L' => {
                for j in 0..k_i {
                    b[j].remove(&i);
                }
            }
            'R' => {
                for j in k_i + 1..n {
                    b[j].remove(&i);
                }
            }
            _ => unreachable!(),
        }
    }

    let mut ans = 1_usize;
    for b_i in b {
        ans *= b_i.len();
        ans %= 998_244_353;
    }

    println!("{}", ans);
}
