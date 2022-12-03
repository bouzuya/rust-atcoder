use std::collections::BTreeMap;

use proconio::input;

fn prime_factorization(n: usize) -> BTreeMap<usize, usize> {
    let mut p = BTreeMap::new();
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
            p.insert(i, c);
        }
    }
    if n != 1 {
        p.insert(n, 1);
    }
    p
}

fn main() {
    input! {
        k: usize,
    };

    let mut pr = 1_usize;
    for x in 2..=10_000_000 {
        pr *= x;
        pr %= k;
        if pr == 0 {
            println!("{}", x);
            return;
        }
    }

    let ps = prime_factorization(k);
    let max_p = ps.iter().rev().next().unwrap();
    let ans = max_p.0.pow(*max_p.1 as u32);
    println!("{}", ans);
}
