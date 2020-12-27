use proconio::input;
use std::{cmp::max, collections::BTreeMap};

fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    let mut p = vec![];
    if n < 2 {
        return p;
    }
    let mut n = n; // shadowing
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut c = 0;
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        if c > 0 {
            p.push((i, c));
        }
    }
    if n != 1 {
        p.push((n, 1));
    }
    p
}

fn main() {
    input! {
        n: usize,
    };
    let mut map = BTreeMap::new();
    for i in 2..=n {
        for (p, q) in prime_factorization(i) {
            let entry = map.entry(p).or_insert(q);
            *entry = max(*entry, q);
        }
    }
    let mut lcm = 1_usize;
    for (p, q) in map {
        lcm *= p.pow(q as u32);
    }
    let ans = lcm + 1;
    println!("{}", ans);
}
