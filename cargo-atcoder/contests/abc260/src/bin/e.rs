use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
    };
    let mut inv = vec![vec![]; m];
    for (i, (a_i, b_i)) in ab.iter().copied().enumerate() {
        inv[a_i].push(i);
        inv[b_i].push(i);
    }

    let mut ans = vec![0_i64; m + 2];
    let mut map = HashMap::new();
    let mut r = 0;
    for l in 0..m {
        while (r < m) && (map.len() != n) {
            for i in inv[r].iter().copied() {
                *map.entry(i).or_insert(0) += 1;
            }
            r += 1;
        }
        if map.len() != n {
            break;
        }
        ans[r - l] += 1;
        ans[m - l + 1] -= 1;
        for i in inv[l].iter().copied() {
            match map.get_mut(&i) {
                Some(e) => {
                    *e -= 1;
                    if *e == 0 {
                        map.remove(&i);
                    }
                }
                None => unreachable!(),
            }
        }
    }

    let mut c = ans[0];
    for a in ans.into_iter().skip(1).take(m) {
        c += a;
        println!("{}", c);
    }
}
