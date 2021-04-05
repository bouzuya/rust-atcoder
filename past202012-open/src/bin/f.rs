use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, Usize1); m],
    };
    let mut max_count = 0;
    for bits in 0..1 << n {
        let select = (0..n).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();
        let mut is_ok = true;
        let mut set = BTreeSet::new();
        for &(a_i, b_i, c_i) in abc.iter() {
            let a = select[a_i];
            let b = select[b_i];
            let c = select[c_i];
            if a && b && c {
                is_ok = false;
            } else if a && b && !c {
                set.insert(c_i);
            } else if a && !b && c {
                set.insert(b_i);
            } else if !a && b && c {
                set.insert(a_i);
            }
        }
        if is_ok {
            max_count = max(max_count, set.len());
        }
    }
    let ans = max_count;
    println!("{}", ans);
}
