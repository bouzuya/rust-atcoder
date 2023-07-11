use std::collections::BTreeMap;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    let mut map = BTreeMap::new();
    for a_i in a {
        map.entry(a_i.to_string().len())
            .or_insert_with(Vec::new)
            .push(a_i);
    }

    let mut selected = vec![];
    for (_, mut v) in map.into_iter().rev() {
        v.sort();
        while let Some(x) = v.pop() {
            selected.push(x);
            if selected.len() == 3 {
                break;
            }
        }
        if selected.len() == 3 {
            break;
        }
    }

    let mut set = vec![];
    let mut is = (0..3).collect::<Vec<_>>();
    loop {
        set.push(format!(
            "{}{}{}",
            selected[is[0]], selected[is[1]], selected[is[2]]
        ));
        if !is.next_permutation() {
            break;
        }
    }
    set.sort();
    set.reverse();

    let ans = &set[0];
    println!("{}", ans);
}
