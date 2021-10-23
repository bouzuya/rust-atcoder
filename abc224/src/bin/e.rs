// TLE
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rca: [(Usize1, Usize1, usize); n],
    };
    let mut col_map = HashMap::new();
    let mut row_map = HashMap::new();
    for (i, (r, c, a)) in rca.iter().copied().enumerate() {
        row_map
            .entry(r)
            .or_insert(BTreeMap::new())
            .entry(a)
            .or_insert(vec![])
            .push(i);
        col_map
            .entry(c)
            .or_insert(BTreeMap::new())
            .entry(a)
            .or_insert(vec![])
            .push(i);
    }

    let mut pq = BinaryHeap::new();
    let mut max = 0;
    for (_, _, a) in rca.iter().copied() {
        max = a.max(max);
    }
    for (i, (_, _, a)) in rca.iter().copied().enumerate() {
        pq.push((Reverse(a), Reverse(0), i));
    }

    let mut dp = BTreeMap::new();
    while let Some((Reverse(_), Reverse(dp_i), i)) = pq.pop() {
        let (r, c, a) = rca[i];
        let v = *dp.get(&(r, c)).unwrap_or(&0);
        if v > dp_i {
            continue;
        }

        let row = row_map.get(&r).unwrap();
        if let Some((_, row_values)) = row.range(1..a).next_back() {
            for j in row_values.iter().copied() {
                let (r, c, _) = rca[j];
                let entry = dp.entry((r, c)).or_insert(0);
                if v + 1 > *entry {
                    *entry = v + 1;
                    pq.push((Reverse(a), Reverse(v + 1), j))
                }
            }
        }

        let col = col_map.get(&c).unwrap();
        if let Some((_, col_values)) = col.range(1..a).next_back() {
            for j in col_values.iter().copied() {
                let (r, c, _) = rca[j];
                let entry = dp.entry((r, c)).or_insert(0);
                if v + 1 > *entry {
                    *entry = v + 1;
                    pq.push((Reverse(a), Reverse(v + 1), j))
                }
            }
        }
    }

    for (r, c, _) in rca {
        println!("{}", *dp.get(&(r, c)).unwrap_or(&0));
    }
}
