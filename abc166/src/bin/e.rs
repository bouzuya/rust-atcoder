use proconio::input;
use std::collections::BTreeMap;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    // j - i = a[i] + a[j] (i < j) が条件より i + a[i] = j - a[j] (i < j)
    let mut l = vec![0_i64; n];
    let mut r = vec![0_i64; n];
    for (i, &a_i) in a.iter().enumerate() {
        l[i] = i as i64 + a_i;
        r[i] = i as i64 - a_i;
    }
    // l[i] = r[j] (i < j) となる (i, j) の個数
    let mut ml: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    let mut mr: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (i, (&l_i, &r_i)) in l.iter().zip(r.iter()).enumerate() {
        ml.entry(l_i).and_modify(|e| e.push(i)).or_insert(vec![i]);
        mr.entry(r_i).and_modify(|e| e.push(i)).or_insert(vec![i]);
    }
    let ans = ml
        .iter()
        .map(|(x, li)| {
            li.iter()
                .map(|&i| {
                    mr.get(&x)
                        .map(|lj| lj.len() - lj.upper_bound(&i))
                        .unwrap_or(0)
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", ans);
}
