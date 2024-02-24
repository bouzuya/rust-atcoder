use std::collections::BTreeMap;

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

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut group = vec![];
    let mut groups = BTreeMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        if a_i == 0 {
            group.push(0);
            continue;
        }
        let ps = prime_factorization(a_i);
        let mut product = 1_usize;
        for (p, c) in ps {
            if c % 2 == 0 {
                continue;
            }
            product *= p;
        }
        group.push(product);
        groups.entry(product).or_insert_with(Vec::new).push(i);
    }
    groups.entry(1).or_insert_with(Vec::new);
    let mut zeros = vec![0; n + 1];
    for (i, a_i) in a.iter().copied().enumerate().rev() {
        zeros[i] = zeros[i + 1] + if a_i == 0 { 1 } else { 0 };
    }

    // println!("{:?}", groups);
    // println!("{:?}", zeros);
    let mut ans = 0_usize;
    for (i, g) in group.iter().copied().enumerate() {
        let count = if g == 0 {
            n - 1 - i
        } else {
            let is = &groups[&g];
            is.len() - 1 - is.lower_bound(&i) + zeros[i]
        };
        // println!("{} {}", a[i], count);
        ans += count;
    }
    println!("{}", ans);
}
