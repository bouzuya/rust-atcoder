use proconio::input;
use proconio::marker::Usize1;
use std::{
    cmp::min,
    collections::{BTreeMap, BTreeSet},
};

fn main() {
    input! {
        n: usize,
        c: i64,
        abc: [(Usize1, usize, i64); n],
    };

    let (map, xyc, xs) = {
        let mut set = BTreeSet::new();
        for &(a_i, b_i, _) in abc.iter() {
            set.insert(a_i);
            set.insert(b_i);
        }
        let mut xs = vec![];
        let mut map = BTreeMap::new();
        for (i, &v) in set.iter().enumerate() {
            *map.entry(v).or_insert(i) = i;
            xs.push(v);
        }
        xs.push(xs[xs.len() - 1] + 1);
        let mut xyc = vec![];
        for &(a_i, b_i, c_i) in abc.iter() {
            xyc.push((*map.get(&a_i).unwrap(), *map.get(&b_i).unwrap(), c_i));
        }
        (map, xyc, xs)
    };

    let mut vs = vec![0; xs.len() - 1];
    for &(x_i, y_i, c) in xyc.iter() {
        vs[x_i] += c;
        vs[y_i] -= c;
    }
    let mut ws = vec![0; xs.len() - 1];
    let mut cur = 0;
    for i in 0..vs.len() {
        cur += vs[i];
        ws[i] = cur;
    }

    let mut ans = 0;
    for (i, &w_i) in ws.iter().enumerate() {
        let l = xs[i + 1] as i64 - xs[i] as i64;
        ans += min(w_i, c) * l;
    }

    println!("{}", ans);
}
