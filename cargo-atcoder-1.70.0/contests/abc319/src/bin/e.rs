use std::collections::{HashMap, HashSet};

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
        n: usize,
        x: usize,
        y: usize,
        pt: [(usize, usize); n - 1],
        capital_q: usize,
        q: [usize; capital_q],
    };

    let mut map = HashMap::new();
    for p in pt
        .iter()
        .copied()
        .map(|(p, _)| p)
        .collect::<HashSet<usize>>()
    {
        for (k, v) in prime_factorization(p) {
            let entry = map.entry(k).or_insert(0);
            *entry = (*entry).max(v);
        }
    }
    let mut lcm = 1_usize;
    for (k, v) in map {
        lcm *= k.pow(v as u32);
    }

    let mut ans = vec![0; lcm];
    for i in 0..lcm {
        let mut sum = i + x;
        for (p, t) in pt.iter().copied() {
            sum += if sum % p == 0 { 0 } else { p - (sum % p) } + t;
        }
        sum += y;
        ans[i] = sum - i;
    }

    for q_i in q {
        println!("{}", q_i + ans[q_i % lcm]);
    }
}
