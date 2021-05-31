use std::collections::BTreeMap;

use proconio::input;

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
        a: usize,
        b: usize,
    };
    if a == b {
        println!("{}", 1);
        return;
    }

    let mut map = BTreeMap::new();
    for x in b + 1..=a {
        let ps = prime_factorization(x);
        for (p, q) in ps {
            *map.entry(p).or_insert(0) += q;
        }
    }
    let mut res = 1_usize;
    for (_, q) in map {
        res *= q + 1;
        res %= 1_000_000_007;
    }
    println!("{}", res);
}
