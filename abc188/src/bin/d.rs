use std::{
    cmp::min,
    collections::{BTreeMap, BTreeSet},
};

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        c: i64,
        abc: [(Usize1, usize, i64); n],
    };
    let mut set = BTreeSet::new();
    for &(a_i, b_i, _) in abc.iter() {
        set.insert(a_i);
        set.insert(b_i);
    }
    let mut mapr = vec![];
    let mut map = BTreeMap::new();
    for (i, &a_or_b) in set.iter().enumerate() {
        map.entry(a_or_b).or_insert(i);
        mapr.push(a_or_b);
    }
    let mut xyc = vec![];
    for &(a_i, b_i, c_i) in abc.iter() {
        xyc.push((*map.get(&a_i).unwrap(), *map.get(&b_i).unwrap(), c_i));
    }
    let mut sum = vec![0; map.len() + 1];
    for &(x_i, y_i, c_i) in xyc.iter() {
        sum[x_i] += c_i;
        sum[y_i] -= c_i;
    }
    for i in 0..map.len() {
        sum[i + 1] += sum[i];
    }
    let mut ans = 0_i64;
    for (i, &s) in sum.iter().enumerate().take(map.len() - 1) {
        ans += min(s, c) * (mapr[i + 1] - mapr[i]) as i64;
    }
    println!("{}", ans);
}
