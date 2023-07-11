use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n],
        mut f: [i64; n],
    };
    let mut sum = 0_i64;
    let mut max_a = 0_i64;
    let mut max_f = 0_i64;
    for i in 0..n {
        let a_i = a[i];
        let f_i = f[i];
        sum += a_i;
        max_a = cmp::max(max_a, a_i);
        max_f = cmp::max(max_f, f_i);
    }
    if k >= sum {
        println!("{}", 0);
        return;
    }
    a.sort_by_key(|&a_i| -a_i);
    f.sort_by_key(|&f_i| f_i);

    let mut ok = max_a * max_f;
    let mut ng = 0;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        let mut sum = 0;
        for i in 0..n {
            let a_i = a[i];
            let f_i = f[i];
            if a_i * f_i <= m {
                continue;
            }
            sum += (a_i * f_i - m + f_i - 1) / f_i;
        }
        if sum <= k {
            ok = m;
        } else {
            ng = m;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
