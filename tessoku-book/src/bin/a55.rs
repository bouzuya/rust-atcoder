use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(usize, i64); q],
    };

    let mut set = BTreeSet::new();
    for (t, x) in tx {
        match t {
            1 => {
                set.insert(x);
            }
            2 => {
                set.remove(&x);
            }
            3 => {
                println!("{}", set.range(x..).next().unwrap_or(&-1));
            }
            _ => unreachable!(),
        }
    }
}
