use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let mut sets = vec![HashSet::new(); m];
    for (i, a_i) in a.iter().copied().enumerate() {
        let i = (i + 1) as i64;
        let a_i = a_i + i;
        let l = if a_i >= 0 { 0 } else { (-a_i + i - 1) / i } as usize;
        let r = if a_i >= n as i64 {
            0
        } else {
            (n as i64 - a_i + i - 1) / i
        }
        .min(m as i64)
        .min(n as i64)
        .max(0) as usize;

        for j in l..r {
            sets[j].insert((a_i + j as i64 * i) as usize);
        }
    }

    for set in sets {
        let mut b = vec![false; set.len() + 1];
        for x in set.iter().copied() {
            if x < set.len() {
                b[x] = true;
            }
        }
        let ans = b.iter().copied().position(|b_i| !b_i).unwrap();
        println!("{}", ans);
    }
}
