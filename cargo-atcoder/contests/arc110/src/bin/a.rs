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
            let entry = map.entry(p).or_insert(0);
            *entry = max(*entry, q);
        }
    }

    let lcm = map
        .iter()
        .map(|(&p, &q)| p.pow(q as u32))
        .product::<usize>();

    let ans = lcm + 1;
    println!("{}", ans);
}
