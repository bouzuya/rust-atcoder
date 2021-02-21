use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [u64; n],
    };
    let mut map: BTreeMap<u64, usize> = BTreeMap::new(); // order by age asc
    for (i, &x_i) in x.iter().enumerate() {
        let rank_i = i + 1;
        if rank_i < k {
            map.entry(x_i).or_insert(rank_i);
            continue;
        }
        if rank_i == k {
            map.entry(x_i).or_insert(rank_i);
        } else {
            assert!(rank_i > k);
            let (&age_k, _) = map.iter().rev().next().unwrap();
            if x_i < age_k {
                map.entry(x_i).or_insert(rank_i);
                map.remove(&age_k);
            }
        }
        let (_, &rank_k) = map.iter().rev().next().unwrap();
        println!("{}", rank_k);
    }
}
