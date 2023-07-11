use std::collections::HashMap;

use proconio::input;

fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    let mut p = vec![];
    if n < 2 {
        return p;
    }
    let mut n = n;
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
        a: [usize; n],
    };
    let mut ps = HashMap::new();
    for a_i in a {
        for (p, q) in prime_factorization(a_i) {
            let e = ps.entry(p).or_insert(q);
            *e = (*e).max(q);
        }
    }
    let ans = ps
        .into_iter()
        .map(|(p, q)| p.pow(q as u32))
        .product::<usize>();
    println!("{}", ans);
}
