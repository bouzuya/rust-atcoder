use std::collections::BTreeMap;

use proconio::input;

fn printk(
    curr: &mut Option<(u64, usize)>,
    map: &BTreeMap<u64, usize>,
    i: usize,
    x_i: u64,
    k: usize,
) {
    if i + 1 == k {
        let (&a, &r) = map.iter().rev().nth(0).unwrap();
        *curr = Some((a, r));
        let (_, curr_rank) = curr.unwrap();
        println!("{}", curr_rank);
    } else {
        let (curr_age, curr_rank) = curr.unwrap();
        if x_i < curr_age {
            let (&a, &r) = map.range(0..curr_age + 1).into_iter().rev().nth(1).unwrap();
            *curr = Some((a, r));
            let (_, curr_rank) = curr.unwrap();
            println!("{}", curr_rank);
        } else {
            assert!(curr_age < x_i);
            println!("{}", curr_rank);
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [u64; n],
    };
    let mut curr = None;
    let mut map: BTreeMap<u64, usize> = BTreeMap::new(); // order by age asc
    for (i, &x_i) in x.iter().enumerate() {
        map.entry(x_i).or_insert(i + 1);
        if i + 1 < k {
            continue;
        }
        printk(&mut curr, &map, i, x_i, k);
    }
}
