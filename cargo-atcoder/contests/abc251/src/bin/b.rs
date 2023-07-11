use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize; n],
    };
    let mut set = HashSet::new();

    // 1
    for a_i in a.iter().copied() {
        if a_i <= w {
            set.insert(a_i);
        }
    }

    // 2
    for i in 0..n {
        for j in i + 1..n {
            if a[i] + a[j] <= w {
                set.insert(a[i] + a[j]);
            }
        }
    }

    // 3
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let v = a[i] + a[j] + a[k];
                if v <= w {
                    set.insert(v);
                }
            }
        }
    }
    let ans = set.len();
    println!("{}", ans);
}
