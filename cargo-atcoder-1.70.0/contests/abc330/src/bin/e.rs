use std::collections::{BTreeSet, HashMap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        ix: [(Usize1, usize); q]
    };

    let mut used = HashMap::new();
    let mut unused = (0..=2 * 100_000).collect::<BTreeSet<usize>>();
    for a_i in a.iter().copied() {
        *used.entry(a_i).or_insert(0) += 1;
        unused.remove(&a_i);
    }

    for (i, x) in ix {
        let count = used.get_mut(&a[i]).unwrap();
        *count -= 1;
        if *count == 0 {
            used.remove(&a[i]);
            unused.insert(a[i]);
        }

        a[i] = x;
        *used.entry(x).or_insert(0) += 1;
        unused.remove(&x);

        let mex = unused.iter().next().unwrap();
        println!("{}", mex);
    }
}
