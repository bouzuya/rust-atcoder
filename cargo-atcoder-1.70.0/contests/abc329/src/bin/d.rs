use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut ans = vec![];
    let mut count = vec![0_usize; n];
    let mut map = vec![BTreeSet::new(); 200_000 + 1];
    let mut keys = BTreeSet::new();
    for a_i in a {
        let entry = &mut map[count[a_i]];
        entry.remove(&a_i);
        if entry.is_empty() {
            keys.remove(&count[a_i]);
        }

        count[a_i] += 1;
        keys.insert(count[a_i]);
        map[count[a_i]].insert(a_i);

        let top = *map[*keys.iter().rev().next().unwrap()].first().unwrap();
        ans.push(top);
    }

    for a in ans {
        println!("{}", a + 1);
    }
}
