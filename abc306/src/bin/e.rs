use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        xy: [(Usize1, usize); q],
    };
    let mut a = vec![0; n];
    let mut sum = 0_usize;
    let mut b1 = BTreeMap::new();
    let mut b2 = BTreeMap::new();
    b1.insert(0, k);
    b2.insert(0, n - k);
    for (x, y) in xy {
        let a_x = a[x];
        a[x] = y;

        let min_b1 = *b1.iter().next().unwrap().0;
        let max_b2 = *b2.iter().rev().next().unwrap().0;

        // a_x の除去
        // a_x はどちら側か
        let is_b1 = if a_x > min_b1 {
            true
        } else if a_x < min_b1 {
            false
        } else {
            assert_eq!(a_x, min_b1);
            if a_x == max_b2 {
                // 両側 → b2 から消す
                false
            } else {
                true
            }
        };
        if is_b1 {
            let entry = b1.get_mut(&a_x).unwrap();
            *entry -= 1;
            if *entry == 0 {
                b1.remove(&a_x);
            }
            sum -= a_x;

            let entry = b2.get_mut(&max_b2).unwrap();
            *entry -= 1;
            if *entry == 0 {
                b2.remove(&max_b2);
            }

            *b1.entry(max_b2).or_insert(0) += 1;
            sum += max_b2;
        } else {
            let entry = b2.get_mut(&a_x).unwrap();
            *entry -= 1;
            if *entry == 0 {
                b2.remove(&a_x);
            }
        }

        *b1.entry(y).or_insert(0) += 1;
        sum += y;

        let min_b1 = *b1.iter().next().unwrap().0;
        let entry = b1.get_mut(&min_b1).unwrap();
        *entry -= 1;
        if *entry == 0 {
            b1.remove(&min_b1);
        }
        sum -= min_b1;
        *b2.entry(min_b1).or_insert(0) += 1;

        println!("{}", sum);
    }
}
