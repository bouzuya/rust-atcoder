use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let set = a.into_iter().collect::<BTreeSet<usize>>();
    let mut ok = 0;
    let mut ng = n + 1;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;

        let mut count = 0_usize;
        for i in 1..=mid {
            if set.contains(&i) {
                count += 1;
            }
        }
        if count + (n - count) / 2 >= mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
