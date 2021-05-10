use proconio::input;
use std::collections::BTreeMap;
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

fn main() {
    input! {
        n: usize,
    };
    let mut map = BTreeMap::new();
    for i in 1..=n {
        for (p, c) in prime_factorization(i) {
            *map.entry(p).or_insert(0) += c;
        }
    }
    let qs = map.iter().map(|(_, &v)| v).collect::<Vec<usize>>();

    // 75 = 3 * 5 * 5
    let mut count = 0;

    {
        let mut ds = vec![3, 5, 5];
        ds.sort();
        loop {
            for (i, &q1) in qs.iter().enumerate() {
                if q1 < ds[0] - 1 {
                    continue;
                }
                for (j, &q2) in qs.iter().enumerate().skip(i + 1) {
                    if q2 < ds[1] - 1 {
                        continue;
                    }
                    for (_, &q3) in qs.iter().enumerate().skip(j + 1) {
                        if q3 < ds[2] - 1 {
                            continue;
                        }
                        count += 1;
                    }
                }
            }
            if !ds.next_permutation() {
                break;
            }
        }
    }

    {
        let mut ds = vec![3 * 5, 5];
        ds.sort();
        loop {
            for (i, &q1) in qs.iter().enumerate() {
                if q1 < ds[0] - 1 {
                    continue;
                }
                for (_, &q2) in qs.iter().enumerate().skip(i + 1) {
                    if q2 < ds[1] - 1 {
                        continue;
                    }
                    count += 1;
                }
            }
            if !ds.next_permutation() {
                break;
            }
        }
    }

    {
        let mut ds = vec![3, 5 * 5];
        ds.sort();
        loop {
            for (i, &q1) in qs.iter().enumerate() {
                if q1 < ds[0] - 1 {
                    continue;
                }
                for (_, &q2) in qs.iter().enumerate().skip(i + 1) {
                    if q2 < ds[1] - 1 {
                        continue;
                    }
                    count += 1;
                }
            }
            if !ds.next_permutation() {
                break;
            }
        }
    }

    {
        let mut ds = vec![3 * 5 * 5];
        ds.sort();
        loop {
            for (_, &q1) in qs.iter().enumerate() {
                if q1 < ds[0] - 1 {
                    continue;
                }
                count += 1;
            }
            if !ds.next_permutation() {
                break;
            }
        }
    }

    println!("{}", count);
}
