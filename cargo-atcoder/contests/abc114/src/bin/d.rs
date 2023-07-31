use std::collections::HashMap;

use proconio::input;
use superslice::Ext;

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

fn dfs(cs: &[usize], qs: &[usize], count: &mut usize, depth: usize, start: usize) {
    if depth == cs.len() {
        *count += 1;
        return;
    }
    for i in start..qs.len() {
        if qs[i] + 1 < cs[depth] {
            continue;
        }
        dfs(cs, qs, count, depth + 1, i + 1);
    }
}

fn main() {
    input! {
        n: usize,
    };
    let qs = {
        let mut pqs = HashMap::new();
        for i in 1..=n {
            for (p, c) in prime_factorization(i) {
                *pqs.entry(p).or_insert(0) += c;
            }
        }
        pqs.values().copied().collect::<Vec<usize>>()
    };

    let f = |mut cs: Vec<usize>| -> usize {
        let mut count = 0_usize;
        cs.sort();
        loop {
            dfs(&cs, &qs, &mut count, 0, 0);
            if !cs.next_permutation() {
                break;
            }
        }
        count
    };

    // p1^3 * p2^5 * p3^5
    let ans = f(vec![3, 5, 5]) + f(vec![3 * 5, 5]) + f(vec![3, 5 * 5]) + f(vec![3 * 5 * 5]);
    println!("{}", ans);
}
