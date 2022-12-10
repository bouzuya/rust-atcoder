use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    };

    let mut pq = BinaryHeap::new();
    let mut map = BTreeMap::new();
    let mut b = a[0..m].iter().copied().collect::<Vec<usize>>();
    b.sort();
    let mut sum = 0_usize;
    for b_i in b[0..k].iter().copied() {
        *map.entry(b_i).or_insert(0) += 1;
        sum += b_i;
    }
    for b_i in b[k..].iter().copied() {
        pq.push(Reverse(b_i));
    }

    let mut ans = vec![];
    ans.push(sum);

    let mut to_ignore = BTreeMap::new();
    for i in (0..n - m + 1).skip(1) {
        let old = a[i - 1];
        let new = a[i - 1 + m];

        let in_map = map.contains_key(&old);
        if in_map {
            sum -= old;
            let count = map.get_mut(&old).unwrap();
            *count -= 1;
            if *count == 0 {
                map.remove(&old);
            }
        } else {
            *to_ignore.entry(old).or_insert(0) += 1;
        }

        pq.push(Reverse(new));
        let to_add = loop {
            let Reverse(x) = pq.pop().unwrap();
            if to_ignore.contains_key(&x) {
                let count = to_ignore.get_mut(&x).unwrap();
                *count -= 1;
                if *count == 0 {
                    to_ignore.remove(&x);
                }
                continue;
            }
            break x;
        };

        if in_map {
            sum += to_add;
            *map.entry(to_add).or_insert(0) += 1;
        } else {
            let max = *map.iter().rev().next().unwrap().0;
            if max <= to_add {
                pq.push(Reverse(to_add));
            } else {
                let count = map.get_mut(&max).unwrap();
                *count -= 1;
                if *count == 0 {
                    map.remove(&max);
                }
                sum -= max;

                sum += to_add;
                *map.entry(to_add).or_insert(0) += 1;

                pq.push(Reverse(max));
            }
        }

        ans.push(sum);
    }

    for a in ans {
        println!("{}", a);
    }
}
