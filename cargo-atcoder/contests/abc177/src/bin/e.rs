use std::collections::HashSet;

use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

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

    let mut pairwise = true;
    let mut set = HashSet::new();
    for a_i in a.iter().copied() {
        for (p, _) in prime_factorization(a_i) {
            if !set.insert(p) {
                pairwise = false;
                break;
            }
        }
    }

    let mut g_a = a[0];
    for a_i in a.iter().copied() {
        g_a = gcd(g_a, a_i);
    }
    let setwise = !pairwise && g_a == 1;

    let ans = if pairwise {
        "pairwise coprime"
    } else if setwise {
        "setwise coprime"
    } else {
        "not coprime"
    };
    println!("{}", ans);
}
