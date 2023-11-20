use std::collections::{BTreeMap, BTreeSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut ans = vec![];
    let mut count = vec![0_usize; n];
    let mut map: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for a_i in a {
        if let Some(set) = map.get_mut(&count[a_i]) {
            set.remove(&a_i);
            if set.is_empty() {
                map.remove(&count[a_i]);
            }
        }

        count[a_i] += 1;
        map.entry(count[a_i]).or_insert(BTreeSet::new()).insert(a_i);

        let top = *map.iter().rev().next().unwrap().1.first().unwrap();
        ans.push(top);
    }

    for a in ans {
        println!("{}", a + 1);
    }
}
