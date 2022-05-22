use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut count = BTreeMap::new();
    for a_i in a.iter().copied() {
        *count.entry(a_i).or_insert(0_usize) += 1;
    }

    let mut curr = 0_usize;
    let mut cumsum = BTreeMap::new();
    for (&k, &v) in count.iter() {
        *cumsum.entry(k).or_insert(0_usize) = curr;
        curr += v;
    }

    let mut ans = 0_usize;
    for a_j in a.iter().copied() {
        let count_all = n;
        let count_a_j = *count.get(&a_j).unwrap();
        let count_a_i = *cumsum.get(&a_j).unwrap();
        let count_a_k = count_all - count_a_i - count_a_j;
        ans += count_a_i * count_a_k;
    }
    println!("{}", ans);
}
