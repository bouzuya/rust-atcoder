use std::cmp::Reverse;

use proconio::input;

fn is_ok(n: usize, m: usize, v: usize, p: usize, a: &[usize], x: usize) -> bool {
    if x <= p {
        return true;
    }
    if a[x - 1] + m < a[p - 1] {
        return false;
    }
    let mut sum = 0;
    for i in 1..=n {
        sum += if i < p {
            m
        } else if (p..x).contains(&i) {
            a[x - 1] + m - a[i - 1]
        } else {
            assert!(x <= i);
            m
        };
    }
    sum >= m * v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        v: usize,
        p: usize,
        mut a: [usize; n],
    };
    a.sort_by_key(|a_i| Reverse(*a_i));
    let mut ok = 1;
    let mut ng = n + 1;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;
        if is_ok(n, m, v, p, &a, x) {
            ok = x;
        } else {
            ng = x;
        }
    }

    let ans = ok;
    println!("{}", ans);
}
