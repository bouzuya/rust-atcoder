use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = 0;
    let mut set = BTreeSet::new();
    for &a_i in a.iter() {
        if set.insert(a_i) {
            let mut ok = true;
            let mut m = a_i * 2;
            while m <= 1_000_000_000 {
                if !set.insert(m) {
                    ok = false;
                }
                m *= 2;
            }
            if ok {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
