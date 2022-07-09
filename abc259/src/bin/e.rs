use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut pes = vec![];
    let mut max = HashMap::new();
    for _ in 0..n {
        input! {
            m_i: usize,
            pe: [(usize, usize); m_i],
        }
        for (p_ij, e_ij) in pe.iter().copied() {
            match max.get_mut(&p_ij) {
                Some((e, _)) => {
                    if *e < e_ij {
                        max.insert(p_ij, (e_ij, 1));
                    } else if *e == e_ij {
                        max.insert(p_ij, (e_ij, 2));
                    }
                }
                None => {
                    max.insert(p_ij, (e_ij, 1));
                }
            }
        }
        pes.push(pe);
    }

    let mut pes2 = BTreeSet::new();
    for pe in pes.iter() {
        let mut pe2 = BTreeSet::new();
        for (p_ij, e_ij) in pe.iter().copied() {
            if max.get(&p_ij).unwrap().0 == e_ij && max.get(&p_ij).unwrap().1 == 1 {
                pe2.insert(p_ij);
            }
        }
        pes2.insert(pe2);
    }

    let ans = pes2.len();
    println!("{}", ans);
}
