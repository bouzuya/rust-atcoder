use std::{cmp::Reverse, collections::BTreeMap};

use proconio::{input, marker::Usize1};

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rca: [(Usize1, Usize1, usize); n],
    };

    let mut a2ps = BTreeMap::new();
    let mut p2i = BTreeMap::new();
    for (i, (r, c, a)) in rca.iter().copied().enumerate() {
        a2ps.entry(Reverse(a))
            .or_insert_with(|| vec![])
            .push((r, c));
        p2i.entry((r, c)).or_insert(i);
    }

    let mut i2d = vec![0; n];
    let mut r2x = vec![0; h];
    let mut r2n = vec![0; w];
    for (_, ps) in a2ps {
        for (r, c) in ps.iter().copied() {
            i2d[p2i[&(r, c)]] = r2x[r].max(r2n[c]);
        }
        for (r, c) in ps.iter().copied() {
            chmax!(r2x[r], i2d[p2i[&(r, c)]] + 1);
            chmax!(r2n[c], i2d[p2i[&(r, c)]] + 1);
        }
    }

    for d in i2d {
        println!("{}", d);
    }
}
