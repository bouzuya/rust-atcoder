use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n + 1],
    };
    if a[0] > 1 {
        println!("-1");
        return;
    }

    let mut v = 1_usize - a[0];
    let mut b = vec![(v, a[0])];
    for &a_i in a.iter().skip(1) {
        if a_i > v * 2 {
            println!("-1");
            return;
        }
        v = v * 2 - a_i;
        b.push((v, a_i));
    }

    let mut c = a[n];
    let mut v = a[n];
    for &(m, l) in b.iter().rev().skip(1) {
        v = cmp::min(m + l, v + l);
        c += v;
    }

    let ans = c;
    println!("{}", ans);
}
