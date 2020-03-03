use proconio::input;
use proconio::marker::{Bytes, Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        mut bv: Bytes,
        q: usize,
        qv: [(usize, Usize1, String); q],
    };

    let mut sv: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 26];
    for i in 0..n {
        let j = (bv[i] - b'a') as usize;
        sv[j].insert(i);
    }

    for (qt, a1, a2) in qv {
        match qt {
            1 => {
                let i = a1;
                let c = a2.bytes().nth(0).unwrap();
                sv[(bv[i] - b'a') as usize].remove(&i);
                bv[i] = c;
                sv[(bv[i] - b'a') as usize].insert(i);
            }
            2 => {
                let l = a1;
                let r = a2.parse::<usize>().unwrap();
                let mut c = 0;
                for s in sv.iter() {
                    c += if s.range(l..r).next().is_some() { 1 } else { 0 };
                }
                println!("{}", c);
            }
            _ => unreachable!(),
        }
    }
}
