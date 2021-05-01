use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(u64, u64); n],
        mut cd: [(u64, u64); n],
    };
    ab.sort();
    cd.sort();

    let mut used = BTreeSet::new();
    for (c_i, d_i) in cd {
        let mut max_b = None;
        for (j, &(a_j, b_j)) in ab.iter().enumerate() {
            if a_j >= c_i {
                break;
            }
            if used.contains(&j) || b_j >= d_i {
                continue;
            }
            match max_b {
                None => max_b = Some((b_j, j)),
                Some((b_k, _)) => {
                    if b_j > b_k {
                        max_b = Some((b_j, j));
                    }
                }
            }
        }
        if let Some((_, j)) = max_b {
            used.insert(j);
        }
    }

    let ans = used.len();
    println!("{}", ans);
}
