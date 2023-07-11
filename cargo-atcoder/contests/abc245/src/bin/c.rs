use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let mut p = if a[0] < b[0] {
        vec![a[0], b[0]]
    } else {
        vec![b[0], a[0]]
    };
    for i in 0..n {
        let a_i = a[i];
        let b_i = b[i];
        let (mn, mx) = if a_i < b_i { (a_i, b_i) } else { (b_i, a_i) };
        let mut set = HashSet::new();
        for p_j in p.iter().copied() {
            if (p_j - mn).abs() as usize <= k {
                set.insert(mn);
            }
        }
        for p_j in p.iter().copied() {
            if (p_j - mx).abs() as usize <= k {
                set.insert(mx);
            }
        }
        if set.is_empty() {
            println!("No");
            return;
        }
        let mut x = vec![];
        for s in set {
            x.push(s);
        }
        p = x;
    }
    println!("Yes");
}
