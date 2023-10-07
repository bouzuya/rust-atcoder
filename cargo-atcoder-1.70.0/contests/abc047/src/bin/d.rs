use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    };
    let t = t / 2 * 2;

    if t == 0 {
        println!("1");
        return;
    }

    let mut min = vec![(a[0], 1); n];
    for (i, a_i) in a.iter().copied().enumerate().skip(1) {
        min[i] = match a_i.cmp(&min[i - 1].0) {
            std::cmp::Ordering::Less => (a_i, 1),
            std::cmp::Ordering::Equal => (a_i, min[i - 1].1 + 1),
            std::cmp::Ordering::Greater => min[i - 1],
        };
    }

    let mut max = vec![(a[n - 1], 1); n];
    for (i, a_i) in a.iter().copied().enumerate().rev().skip(1) {
        max[i] = match a_i.cmp(&max[i + 1].0) {
            std::cmp::Ordering::Less => max[i + 1],
            std::cmp::Ordering::Equal => (a_i, max[i + 1].1 + 1),
            std::cmp::Ordering::Greater => (a_i, 1),
        };
    }

    let mut max_profit_set = HashSet::new();
    let mut max_profit = 0_usize;
    for i in 0..n - 1 {
        let profit = max[i].0 - min[i].0;
        match profit.cmp(&max_profit) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => {
                max_profit_set.insert((max[i], min[i]));
            }
            std::cmp::Ordering::Greater => {
                max_profit = profit;
                max_profit_set.clear();
                max_profit_set.insert((max[i], min[i]));
            }
        }
    }

    let ans = max_profit_set.len();
    println!("{}", ans);
}
