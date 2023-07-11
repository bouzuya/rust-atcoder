use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        _: usize,
        a: Usize1,
        b: Usize1,
        k: usize,
        p: [Usize1; k],
    };
    let mut set = BTreeSet::new();
    set.insert(a);
    set.insert(b);
    for p_i in p {
        if !set.insert(p_i) {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
