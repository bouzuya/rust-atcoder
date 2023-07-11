use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    };
    if m == 0 {
        println!("{}", 1);
        return;
    }
    if m >= n {
        println!("{}", 0);
        return;
    }
    a.sort();
    a.push(0);
    a.push(n + 1);
    a.sort();

    let k = {
        let mut min_v = n;
        let mut p = a[0];
        for &a_i in a.iter().skip(1) {
            if a_i - p > 1 {
                min_v = cmp::min(min_v, a_i - p - 1);
            }
            p = a_i;
        }
        min_v
    };
    let mut count = 0;
    let mut p = a[0];
    for &a_i in a.iter().skip(1) {
        if a_i - p > 1 {
            let d = a_i - p - 1;
            count += (d + k - 1) / k;
        }
        p = a_i;
    }
    let ans = count;
    println!("{}", ans);
}
